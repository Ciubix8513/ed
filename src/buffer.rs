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
impl From<(usize, Option<usize>)> for CommandIndex {
    fn from(value: (usize, Option<usize>)) -> Self {
        Self {
            begining: value.0,
            end: value.1,
        }
    }
}

///Main buffer that is being edited
#[derive(Default)]
pub struct Buffer {
    ///The actual text of the file, stored as an array of string for easier modification
    pub lines: Vec<String>,
    pub cursor: usize,
    pub modified: bool,
    pub filename: String,
    pub marker: usize,
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

    fn parse_index<'a>(
        &self,
        ind: &'a str,
        //This is some serious type fuckery
    ) -> (Result<Option<(usize, Option<usize>)>, ()>, &'a str) {
        //Valid index chars:
        //. = current line
        //$ = last line
        //0 - 9 = digits
        //+ - = modification of other symbols
        //, = address range separation
        //; = address range additional char N; = N,$Index
        //x = Marker
        //Manual parsing?
        let mut parsing_index: usize = 0;
        let mut offset = 0;
        let mut first_index = None;
        let mut second_index = None;
        let mut number_buffer = String::new();
        let mut first_index_aquiered = false;
        let mut complete_index = None;

        for c in ind.chars() {
            match c {
                '.' => {
                    if first_index.is_none() {
                        first_index = Some(self.cursor);
                    } else {
                        second_index = Some(self.cursor);
                    }
                }
                '$' => {
                    if first_index.is_none() {
                        first_index = Some(self.lines.len() - 1);
                    } else {
                        second_index = Some(self.lines.len() - 1);
                    }
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    number_buffer.push(c);
                }
                ',' | ';' => {
                    if number_buffer.is_empty() {
                        complete_index = if c == ',' {
                            Some((self.cursor, Some(self.lines.len())))
                        } else {
                            Some((self.cursor, Some(self.lines.len())))
                        };
                    }
                }
                'x' => {
                    if number_buffer.is_empty() {
                        second_index = Some(self.marker);
                    } else {
                        second_index = Some(self.marker);
                    }
                }
                '+' => {
                    offset += 1;
                }
                '-' => {
                    offset -= 1;
                }
                //Reached the end of the index
                _ => {
                    if number_buffer.is_empty() {
                        return (
                            Ok(first_index.map(|i| (i, second_index))),
                            &ind[parsing_index..],
                        );
                    } else {
                        if second_index.is_some() {
                            return (Err(()), &ind[parsing_index..]);
                        }
                        let index = number_buffer.parse::<usize>().ok().map(|i| i + offset);
                        // No need to clear this, since there will be no more parsing
                        // number_buffer.clear();
                        // offset = 0;
                        return if first_index.is_some() {
                            (
                                Ok(Some((first_index.unwrap(), index))),
                                &ind[parsing_index..],
                            )
                        } else {
                            (Ok(index.map(|i| (i, None))), &ind[parsing_index..])
                        };
                    }
                }
            }
            parsing_index += 1;
        }
        (Ok(None), ind)
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
            //God, the type fuckery is real
            'p' => Operation::Print(index.ok().map(|i| i.map(Into::into)).flatten()),
            _ => Operation::Error("Unknown command"),
        }
    }
}

#[test]
fn index_test() {
    let buffer = Buffer::default();
    let command = "1,10p";
    let (index, command) = buffer.parse_index(command);

    assert_eq!(index, Ok(Some((1, Some(10)))));
    assert_eq!(command, "p");
}
