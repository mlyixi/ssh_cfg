use ssh_cfg::{SshConfigParser, SshOptionKey};
use std::path::Path;

fn parse_ssh_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = shellexpand::tilde("~/.ssh/config");
    let config_path = Path::new(config_path.as_ref());
    let ssh_config = SshConfigParser::parse(config_path)?;

    // Print first host config
    if let Some((first_host, host_config)) = ssh_config.iter().next() {
        println!("Host: {}", first_host);

        // Print its configured SSH key if any
        if let Some(identity_file) = host_config.get(&SshOptionKey::IdentityFile) {
            println!("  {} {}", SshOptionKey::IdentityFile, identity_file);
        }
    }

    // Print all host configs
    println!();
    println!("{:#?}", ssh_config);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    parse_ssh_config()
}
