use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct UpCli {
    /// Whether the system should be rebooted after the update.
    #[arg(short, long)]
    pub reboot: bool,

    #[arg(short, long)]
    pub config: Option<PathBuf>,
}
