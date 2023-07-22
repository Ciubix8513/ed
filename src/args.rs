#![allow(clippy::struct_excessive_bools)]
use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(version, about, author)]
pub struct Args {
    ///use extended regular expressions
    #[arg(short = 'E', long)]
    pub extended_regexp: bool,
    ///run in compatibility mode
    #[arg(short = 'G', long)]
    pub traditional: bool,
    ///exit with 0 status even if a command fails
    #[arg(short, long)]
    pub lose_exit_status: bool,
    #[arg(short, long)]
    pub prompt: Option<String>,
    #[arg(short, long)]
    pub verbose: bool,
    pub file: Option<PathBuf>,
}
