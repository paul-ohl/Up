use std::{
    io::{stdout, Stdout, Write},
    process::{Command, Output},
    sync::{Arc, Mutex},
};

use color_eyre::eyre::Result;
use crossterm::{cursor, QueueableCommand};
use rayon::prelude::*;

/// # Errors
/// Returns an error if the configuration file cannot be read.
/// # Panics
/// Panics if joining the async handles fails
#[allow(clippy::cast_possible_truncation)]
pub fn run_commands(commands: &[(String, String)]) -> Result<()> {
    let stdout_mutex = Arc::new(Mutex::new(stdout()));
    let initial_cursor_line = cursor::position()?.1;
    let max_command_length = commands
        .iter()
        .map(|(name, _)| name)
        .map(String::len)
        .max()
        .unwrap_or(0) as u16;
    let vec_commands: Vec<_> = commands.iter().collect();

    vec_commands
        .into_par_iter()
        .enumerate()
        .for_each(|(i, (name, command))| {
            let y_pos = initial_cursor_line + i as u16;
            print_to_screen(0, y_pos, name, &stdout_mutex);
            if let Ok(output) = execute_command(command) {
                if output.status.success() {
                    print_to_screen(max_command_length + 1, y_pos, "OK ✔️", &stdout_mutex);
                } else {
                    print_to_screen(
                        max_command_length + 1,
                        y_pos,
                        format!(
                            "KO ❌ {}",
                            stderr_first_line(output.stderr).unwrap_or_else(|_| String::new())
                        ),
                        &stdout_mutex,
                    );
                    // Write the error message to a log file
                }
            } else {
                print_to_screen(
                    max_command_length + 1,
                    y_pos,
                    "Failed to execute command",
                    &stdout_mutex,
                );
            }
        });

    go_to_last_line(initial_cursor_line + commands.len() as u16, &stdout_mutex);
    Ok(())
}

fn execute_command(command: &str) -> Result<Output> {
    let command_args: Vec<&str> = command.split_whitespace().collect();
    let mut command_builder = Command::new(command_args[0]);
    for arg in &command_args[1..] {
        command_builder.arg(arg);
    }
    Ok(command_builder.output()?)
}

fn go_to_last_line(last_line: u16, stdout_mutex: &Arc<Mutex<Stdout>>) {
    let mut stdout = stdout_mutex
        .lock()
        .expect("Failed to lock stdout in go_to_last_line.");
    stdout
        .queue(cursor::MoveTo(0, last_line))
        .expect("Failed to move cursor to last line.");
    let _ = stdout.flush();
}

fn print_to_screen(
    x_pos: u16,
    y_pos: u16,
    content: impl std::fmt::Display,
    stdout_mutex: &Arc<Mutex<Stdout>>,
) {
    if let Ok(mut stdout) = stdout_mutex.lock() {
        if stdout.queue(cursor::MoveTo(x_pos, y_pos)).is_ok() {
            write!(stdout, "{content}").unwrap();
            let _ = stdout.flush();
        }
    }
}

fn stderr_first_line(stderr: Vec<u8>) -> Result<String> {
    Ok(String::from_utf8(stderr)?
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_string())
}
