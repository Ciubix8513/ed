#![allow(clippy::inherent_to_string)]
use std::path::PathBuf;

pub enum Operation {
    Quit,
    Error(&'static str),
    TogglePrompt,
    Insert,
    Append,
    Write(String),
    ToggleVerbose,
    Print(Option<CommandIndex>),
}

pub struct CommandIndex {
    pub begining: usize,
    pub end: Option<usize>,
}

///Main buffer that is being edited
#[derive(Default)]
pub struct Buffer {
    ///The actual text of the file, stored as an array of string for easier modification
    pub lines: Vec<String>,
    pub cursor: usize,
    pub modified: bool,
    pub filename: String,
}
pub fn string_to_lines(input: &str) -> Vec<String> {
    let mut o = input
        .split('\n')
        .map(Into::<String>::into)
        .collect::<Vec<_>>();
    o.pop();
    o
}

impl Buffer {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self {
            lines: path.map_or(Vec::new(), |path| {
                if !path.exists() {
                    println!("{}: No such file or directory", path.display());
                }
                if path.is_dir() {
                    println!("{}: Is a directory", path.display());
                }
                let file = std::fs::read(path).unwrap();
                println!("{}", file.len());
                string_to_lines(&String::from_utf8(file).unwrap())
            }),
            ..Default::default()
        }
    }
    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }

    fn parse_index<'a>(&self, ind: &'a str) -> (Option<CommandIndex>, &'a str) {
        (None, ind)
    }

    pub fn parse_command(&self, command: &str) -> Operation {
        let (index, command) = self.parse_index(command);
        match command.chars().next().unwrap_or(' ') {
            'q' | 'Q' => Operation::Quit,
            'P' => Operation::TogglePrompt,
            'i' => Operation::Insert,
            'a' => Operation::Append,
            'w' => Operation::Write(if command.len() >= 3 {
                command[2..].trim_start().into()
            } else {
                String::new()
            }),
            'H' => Operation::ToggleVerbose,
            'p' => Operation::Print(index),
            _ => Operation::Error("Unknown command"),
        }
    }
}
