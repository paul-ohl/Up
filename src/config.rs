use core::panic;
use std::{collections::HashMap, error::Error, fmt, path::PathBuf};

use config::{Config, File, FileFormat};

use crate::UpCli;

#[cfg(test)]
use crate::test::dirs::config_dir;
#[cfg(not(test))]
use dirs::config_dir;

/// # Errors
/// Returns an error if the configuration file cannot be found or read.
pub fn get_commands(arguments: &UpCli) -> Result<Vec<(String, String)>, ConfigError> {
    let config_path = get_config_path(&arguments.config)?;

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
        .map(|(command_name, command)| (command_name.replace('_', " "), command))
        .collect();

    if config.is_empty() {
        return Err(ConfigError::FileEmpty(format!(
            "File {config_path} is empty, or all its commands are empty",
        )));
    }
    Ok(config)
}

fn get_config_path(config_file_path: &Option<PathBuf>) -> Result<String, ConfigError> {
    let path = if let Some(path) = &config_file_path {
        path.clone()
    } else {
        let mut path = config_dir().ok_or_else(|| {
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
    FsError(String),
    NotFound(String),
    FileError(String),
    FileEmpty(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(s) | Self::FileEmpty(s) | Self::FileError(s) | Self::FsError(s) => {
                write!(f, "Error reading config: {s}")
            }
        }
    }
}

impl Error for ConfigError {}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    use crate::UpCli;

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

    #[test]
    fn get_file_path_works_properly() {
        let path = get_config_path(&Some("/tmp/config.toml".into()));
        assert!(path.is_ok());
        assert_eq!(path.unwrap(), "/tmp/config.toml");
    }

    // This test may fail on Windows
    #[test]
    fn get_config_path_defaults_to_standard_config_directory() {
        let path = get_config_path(&None);
        let check_path = PathBuf::from("/tmp").join("up").join("commands.toml");
        assert!(path.is_ok());
        assert_eq!(path.unwrap(), check_path.to_str().unwrap());
    }

    #[test]
    fn get_config_works_with_valid_input() {
        let arguments = UpCli {
            reboot: false,
            config: Some(PathBuf::from("./test/valid_configs/valid_config.toml")),
        };
        let result = get_commands(&arguments);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.contains(&(String::from("multiple words"), String::from("echo valid"))));
        assert!(result.contains(&(String::from("word"), String::from("echo valid"))));
    }

    #[test]
    fn test_error_display() {
        let error = ConfigError::NotFound("file1.toml".to_string());
        assert_eq!(error.to_string(), "Error reading config: file1.toml");

        let error = ConfigError::FileError("file1.toml".to_string());
        assert_eq!(error.to_string(), "Error reading config: file1.toml");

        let error = ConfigError::FileEmpty("file1.toml".to_string());
        assert_eq!(error.to_string(), "Error reading config: file1.toml");
    }
}
