use std::collections::HashMap;

use color_eyre::eyre::{eyre, Result};
use config::{Config, File, FileFormat};

use crate::UpCli;

/// # Errors
/// Returns an error if the configuration file cannot be found or read.
pub fn get_commands(config: &UpCli) -> Result<Vec<(String, String)>> {
    let config_path = get_config_path(config)?;

    let config = Config::builder()
        .add_source(File::new(&config_path, FileFormat::Toml))
        .build()?;
    let config: HashMap<String, String> = config.try_deserialize()?;
    let config: Vec<(String, String)> = config.into_iter().collect();

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
