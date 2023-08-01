#![allow(clippy::too_many_lines)]
use std::{
    io::{ErrorKind, Write},
    process::ExitCode,
};

use buffer::string_to_lines;
#[allow(unused)]
use buffer::Buffer;
use clap::Parser;

use crate::{args::Args, buffer::Operation};

mod args;
mod buffer;
enum Mode {
    Command,
    Edit,
}

fn main() -> ExitCode {
    let args = Args::parse();

    //The main buffer
    let mut buffer = Buffer::new(args.file);

    let stdin = std::io::stdin();
    let mut display_prompt = args.prompt.is_some();

    let prompt = args.prompt.unwrap_or_else(|| "*".into());
    let mut command = String::new();
    let mut mode = Mode::Command;
    let mut verbose = args.verbose;
    let mut editing_buffer = String::new();
    let mut tried_quit = false;

    let mut last_error = String::new();

    loop {
        command.clear();
        match mode {
            Mode::Command => {
                if display_prompt {
                    print!("{prompt}");
                }
                std::io::stdout().flush().unwrap();
                stdin.read_line(&mut command).unwrap();
                if command.chars().last().is_none() {
                    command.push('q');
                }
                match buffer.parse_command(command.trim_end_matches('\n')) {
                    Operation::Quit => {
                        if !buffer.modified || tried_quit {
                            return ExitCode::SUCCESS;
                        }
                        tried_quit = true;
                        last_error = "Warning: buffer modified".into();
                        println!("?");
                        continue;
                    }
                    Operation::Error(e) => {
                        last_error = e.into();
                        println!("?");
                    }
                    Operation::TogglePrompt => {
                        display_prompt = !display_prompt;
                    }
                    Operation::Insert => {
                        if buffer.cursor > 0 {
                            buffer.cursor -= 1;
                        }
                        mode = Mode::Edit;
                    }
                    Operation::Append => mode = Mode::Edit,
                    Operation::Write(file) => {
                        if file.is_empty() && buffer.filename.is_empty() {
                            last_error = "No current filename".into();
                            println!("?");
                        } else {
                            if !file.is_empty() {
                                buffer.filename = file;
                            }
                            match std::fs::File::create(&buffer.filename) {
                                Ok(mut f) => match f.write(buffer.to_string().as_bytes()) {
                                    Ok(i) => {
                                        println!("{i}");
                                    }
                                    Err(e) => {
                                        println!("?");
                                        last_error = e.to_string();
                                    }
                                },
                                Err(e) => {
                                    let kind = e.kind();
                                    if kind == ErrorKind::PermissionDenied {
                                        last_error =
                                            format!("{}: permission denied", buffer.filename);
                                    } else if kind == ErrorKind::NotFound {
                                        last_error = format!(
                                            "{}: No such file or directory",
                                            buffer.filename
                                        );
                                    } else if kind == ErrorKind::InvalidInput {
                                        //IDs what the original name is, but eh, it's fine, idk how
                                        //you would even get this error
                                        last_error =
                                            format!("{}: invalid filename", buffer.filename);
                                    } else {
                                        last_error = format!("{}: {} \n btw how did you do this? srsly, i'm curious, feel free to msg me",buffer.filename,e);
                                    }
                                    println!("?");
                                }
                            };
                        }
                        buffer.modified = false;
                    }
                    Operation::ToggleVerbose => verbose = !verbose,
                    Operation::Print(_) => println!("{:?}", buffer.lines),
                }
                tried_quit = false;
            }
            Mode::Edit => {
                stdin.read_line(&mut command).unwrap();
                if command.trim().eq(".") {
                    // buffer.lines.push(editing_buffer.clone());
                    println!("Adding {editing_buffer} at {}", buffer.cursor);
                    buffer.lines.splice(
                        buffer.cursor..buffer.cursor,
                        string_to_lines(&editing_buffer),
                    );
                    buffer.cursor = buffer.lines.len();
                    editing_buffer.clear();
                    mode = Mode::Command;
                    buffer.modified = true;
                    continue;
                }
                editing_buffer.push_str(&command);
            }
        }
        if verbose && !last_error.is_empty() {
            println!("{last_error}");
            last_error.clear();
        }
    }
}
