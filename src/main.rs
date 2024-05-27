use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use up::{config::get_commands, run_commands, UpCli};

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli_params = UpCli::parse();
    let commands = get_commands(&cli_params)?;

    run_commands(&commands)?;
    if cli_params.reboot {
        reboot_system()?;
    }
    Ok(())
}

fn reboot_system() -> Result<()> {
    match system_shutdown::reboot() {
        Ok(()) => Ok(()),
        Err(e) => Err(eyre!("Failed to reboot the system: {e}")),
    }
}
