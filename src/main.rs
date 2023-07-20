use buffer::Buffer;
use clap::Parser;

use crate::args::Args;

mod args;
mod buffer;

fn main() {
    let args = Args::parse();

    //The main buffer
    let buffer = Buffer::new(args.file);
}
