use core::panic;
use std::{collections::HashMap, error::Error, fmt};

use config::{Config, File, FileFormat};

use crate::UpCli;

/// # Errors
/// Returns an error if the configuration file cannot be found or read.
pub fn get_commands(arguments: &UpCli) -> Result<Vec<(String, String)>, ConfigError> {
    let config_path = get_config_path(arguments)?;

    let config = Config::builder()
        .add_source(File::new(&config_path, FileFormat::Toml))
        .build()
        .map_err(|e| ConfigError::NotFound(format!("Could not find file: {config_path}\n{e}")))?;
    let config: HashMap<String, String> = config
        .try_deserialize()
        .map_err(|e| ConfigError::FileError(format!("Could not read file {config_path}.\n{e}",)))?;
    let config: Vec<(String, String)> = config
        .into_iter()
        .filter(|(_, command)| !command.is_empty())
        .collect();

    if config.is_empty() {
        return Err(ConfigError::FileEmpty(format!(
            "File {config_path} is empty, or all its commands are empty",
        )));
    }
    Ok(config)
}

fn get_config_path(config_file_path: &UpCli) -> Result<String, ConfigError> {
    let path = if let Some(path) = &config_file_path.config {
        path.clone()
    } else {
        let mut path = dirs::config_dir().ok_or_else(|| {
            ConfigError::NotFound("Could not find a standard config location".to_string())
        })?;
        path.push("up");
        path.push("commands.toml");
        path
    };

    path.to_str()
        .ok_or_else(|| panic!("This is an unexpected error that shouldn't happen. Please report this on Github. In the mean time, you can use up with the `-c` option."))
        .map(std::string::ToString::to_string)
}

#[derive(Debug)]
pub enum ConfigError {
    NotFound(String),
    FileError(String),
    FileEmpty(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(s) | Self::FileEmpty(s) | Self::FileError(s) => {
                write!(f, "Error reading config: {s}")
            }
        }
    }
}

impl Error for ConfigError {}

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
                result.is_err(),
                "get_commands for {wrong_file} should have failed",
            );
        }
    }
}
