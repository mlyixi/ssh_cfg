use std::{fs, path::Path};

use crate::{ConfigError, Error, SshConfig, SshHostConfig, SshOptionKey};

/// Parses SSH configuration file into [`SshConfig`].
pub struct SshConfigParser;

// See https://github.com/substack/libssh/blob/master/src/config.c
impl SshConfigParser {
    /// Returns the parsed SSH configuration.
    pub fn parse(path: &Path) -> Result<SshConfig, Error> {
        let contents = Self::ssh_config_contents(&path)?;
        let ssh_config = Self::parse_config_contents(&contents)?;

        Ok(ssh_config)
    }

    fn ssh_config_contents(path: &Path) -> Result<String, Error> {
        fs::read_to_string(path).map_err(|error| Error::SshConfigRead {
            path: path.to_path_buf(),
            error,
        })
    }

    /// Parses SSH configuration in memory.
    ///
    /// # Parameters
    ///
    /// * `ssh_config_contents`: The SSH configuration.
    pub fn parse_config_contents(ssh_config_contents: &str) -> Result<SshConfig, Error> {
        let mut errors = Vec::new();
        let kv_pairs = Self::kv_pairs(ssh_config_contents, &mut errors).into_iter();

        let mut ssh_config = SshConfig::default();
        let mut current_host = None;
        let mut ssh_host_config = SshHostConfig::default();
        for (key, value) in kv_pairs {
            let ssh_option_key = match key.parse::<SshOptionKey>() {
                Ok(ssh_option_key) => ssh_option_key,
                Err(error) => {
                    errors.push(error);
                    continue;
                }
            };

            if let SshOptionKey::Host = ssh_option_key {
                if let Some(current_host) = current_host.take() {
                    ssh_config.insert(current_host, ssh_host_config);

                    // Initialize new config for the next host.
                    ssh_host_config = SshHostConfig::default();
                }

                current_host = Some(value.to_string());
            } else if current_host.is_none() {
                errors.push(ConfigError::SshOptionBeforeHost {
                    option: ssh_option_key,
                    value: value.to_string(),
                });
            } else {
                ssh_host_config.insert(ssh_option_key, value.to_string());
            }
        }

        // Insert the final host's config.
        if let Some(current_host) = current_host.take() {
            ssh_config.insert(current_host, ssh_host_config);
        }

        if errors.is_empty() {
            Ok(ssh_config)
        } else {
            Err(Error::ConfigErrors { errors })
        }
    }

    fn kv_pairs<'f>(
        ssh_config_contents: &'f str,
        config_errors: &mut Vec<ConfigError>,
    ) -> Vec<(&'f str, &'f str)> {
        ssh_config_contents
            .lines()
            // Only consider content before the first `#`
            .map(|line| line.split_once('#').map_or(line, |split| split.0))
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                // Key and value are either separated by `=`, or whitespace.
                // We try split by `=` first, and if there are two tokens, then those are
                // the key and value pair. Otherwise we split the first token by ` `.
                let kv_pair = Self::kv_split_by(line, '=').or_else(|| Self::kv_split_by(line, ' '));
                if kv_pair.is_none() {
                    config_errors.push(ConfigError::KeyValueNotFound {
                        line: line.to_string(),
                    });
                }

                kv_pair
            })
            .collect::<Vec<_>>()
    }

    /// Returns the key and value split by the given character.
    fn kv_split_by(line: &str, separator: char) -> Option<(&str, &str)> {
        let mut kv_split = line.splitn(2, separator);
        let key = kv_split.next();
        let value = kv_split.next();

        match (key, value) {
            (Some(key), Some(value)) => Some((key.trim(), value.trim())),
            (Some(_), None) => None,
            _ => unreachable!("Empty lines are filtered."),
        }
    }
}
