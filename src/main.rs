use clap::Parser;
use up::{config::get_commands, run_commands, UpCli};

fn main() {
    let cli_params = UpCli::parse();
    match get_commands(&cli_params) {
        Ok(commands) => {
            if let Err(e) = run_commands(&commands) {
                eprintln!("{e}");
            } else if cli_params.reboot {
                reboot_system();
            }
        }
        Err(e) => eprintln!("{e}"),
    }
}

fn reboot_system() {
    match system_shutdown::reboot() {
        Ok(()) => println!("Rebooting..."),
        Err(e) => eprintln!("Failed rebooting.\n{e}"),
    }
}
