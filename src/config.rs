use std::collections::HashMap;

use color_eyre::eyre::{eyre, Result};
use config::{Config, File, FileFormat};

use crate::UpCli;

/// # Errors
/// Returns an error if the configuration file cannot be found or read.
pub fn get_commands(arguments: &UpCli) -> Result<Vec<(String, String)>> {
    let config_path = get_config_path(arguments)?;

    let config = Config::builder()
        .add_source(File::new(&config_path, FileFormat::Toml))
        .build()?;
    let config: HashMap<String, String> = config.try_deserialize()?;
    let config: Vec<(String, String)> = config
        .into_iter()
        .filter(|(_, command)| !command.is_empty())
        .collect();

    if config.is_empty() {
        return Err(eyre!("No commands found in the configuration file."));
    }
    Ok(config)
}

fn get_config_path(config_file_path: &UpCli) -> Result<String> {
    let path = if let Some(path) = &config_file_path.config {
        path.clone()
    } else {
        let mut path = dirs::config_dir()
            .ok_or_else(|| eyre!("Failed to get the configuration directory path."))?;
        path.push("up");
        path.push("commands.toml");
        path
    };

    path.to_str()
        .ok_or_else(|| eyre!("Failed to convert the path to a string."))
        .map(std::string::ToString::to_string)
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::UpCli;

    use super::get_commands;

    #[test]
    fn get_config_fails_properly_with_invalid_config_file() {
        let files = [
            "empty_command.toml",
            "empty_config.toml",
            "no_command_name.toml",
            "no_command.toml",
        ];
        for wrong_file in files {
            let arguments = UpCli {
                reboot: false,
                config: Some(PathBuf::from(
                    "./test/invalid_configs/".to_string() + wrong_file,
                )),
            };
            let result = get_commands(&arguments);
            assert!(
                !result.is_ok(),
                "get_commands for {wrong_file} should have failed",
            );
        }
    }
}
