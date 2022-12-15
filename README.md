# 🌐 synchronous SSH Config parser

Parse ssh config file synchronously.

```rust
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
```

Currently values are stored as `String`s. Ideally we would parse them into a
strong data model.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
