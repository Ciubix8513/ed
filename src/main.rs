use std::{io::Write, process::ExitCode};

#[allow(unused)]
use buffer::Buffer;
use clap::Parser;

use crate::{args::Args, commands::parse_command};

mod args;
mod buffer;
mod commands;
enum Mode {
    Command,
    Edit,
}

fn main() -> ExitCode {
    let args = Args::parse();

    //The main buffer
    let mut buffer = Buffer::new(args.file);

    let stdin = std::io::stdin();

    let mut prompt = args.prompt.unwrap_or_default();
    let mut command = String::new();
    let mut mode = Mode::Command;
    let mut verbose = args.verbose;
    let mut editing_buffer = String::new();

    loop {
        command.clear();
        match mode {
            Mode::Command => {
                print!("{prompt}");
                std::io::stdout().flush().unwrap();
                stdin.read_line(&mut command).unwrap();
                match parse_command(&command) {
                    commands::Operation::Quit => return ExitCode::SUCCESS,
                    commands::Operation::Error(e) => {
                        if verbose {
                            println!("{e}");
                        }
                        println!("?")
                    }
                    commands::Operation::SetPrompt(p) => prompt = p,
                    commands::Operation::Insert => {
                        if buffer.cursor > 0 {
                            buffer.cursor -= 1;
                        }
                        mode = Mode::Edit
                    }
                    commands::Operation::Append => mode = Mode::Edit,
                }
            }
            Mode::Edit => {
                stdin.read_line(&mut command).unwrap();
                if editing_buffer.trim().eq(".") {
                    buffer.buffer.push_str(&editing_buffer);
                    mode = Mode::Command;
                    editing_buffer.clear();
                    continue;
                }
                editing_buffer.push_str(&command);
            }
        }
        // if command.contains('q') {
        //     return ExitCode::SUCCESS;
        // }
        // if command.is_empty() {
        //     println!("?");
        //     continue;
        // }
    }
}
