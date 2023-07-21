use std::{io::Write, process::ExitCode};

#[allow(unused)]
use buffer::Buffer;
use clap::Parser;

use crate::{args::Args, commands::parse_command};

mod args;
mod buffer;
mod commands;

fn main() -> ExitCode {
    let args = Args::parse();

    //The main buffer
    let mut buffer = Buffer::new(args.file);

    let stdin = std::io::stdin();

    let mut prompt = args.prompt.unwrap_or_default();
    let mut command = String::new();

    loop {
        print!("{prompt}");
        std::io::stdout().flush().unwrap();
        command.clear();
        stdin.read_line(&mut command).unwrap();
        match parse_command(&command) {
            commands::Operation::Quit => return ExitCode::SUCCESS,
            commands::Operation::Error => println!("?"),
            commands::Operation::SetPrompt(p) => prompt = p,
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
