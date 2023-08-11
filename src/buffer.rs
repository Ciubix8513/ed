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
        let mut complete_index = None;

        for c in ind.chars() {
            match c {
                '.' => {
                    if !number_buffer.is_empty() {
                        println!("{:?}", number_buffer.chars());
                        return (Err(()), &ind[parsing_index..]);
                    }
                    number_buffer = self.cursor.to_string();
                }
                '$' => {
                    if !number_buffer.is_empty() {
                        println!("{:?}", number_buffer.chars());
                        return (Err(()), &ind[parsing_index..]);
                    }
                    number_buffer = self.lines.len().to_string();
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    number_buffer.push(c);
                }
                ',' | ';' => {
                    if number_buffer.is_empty() {
                        if offset != 0 {
                            println!("Error at {}", line!());
                            return (
                                Err(()),
                                &ind[parsing_index
                                    + if ind.len() >= parsing_index { 1 } else { 0 }..],
                            );
                        }
                        complete_index = if c == ',' {
                            Some((self.cursor, Some(self.lines.len())))
                        } else {
                            Some((0, Some(self.lines.len())))
                        };
                        parsing_index += 1;
                        continue;
                    }

                    // we can safely unwrap this one because we push only numbers
                    let index: i32 = number_buffer.parse().unwrap();

                    if -offset >= index || index + offset > self.lines.len() as i32 {
                        println!("Offset {offset}, index {index}, len {}", self.lines.len());

                        println!("Error at {}", line!());
                        return (
                            Err(()),
                            &ind[parsing_index + if ind.len() >= parsing_index { 1 } else { 0 }..],
                        );
                    }
                    if c == ';' {
                        return (
                            Ok(Some(((index + offset) as usize, Some(self.lines.len())))),
                            &ind[parsing_index + if ind.len() >= parsing_index { 1 } else { 0 }..],
                        );
                    }
                    first_index = Some((index + offset) as usize);
                    number_buffer.clear();
                    offset = 0;
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
                    if complete_index.is_some() {
                        if !number_buffer.is_empty() || offset != 0 {
                            println!("Error at {}", line!());
                            return (Err(()), &ind[parsing_index..]);
                        }
                        return (Ok(complete_index), &ind[parsing_index..]);
                    }
                    if number_buffer.is_empty() {
                        return (
                            Ok(first_index.map(|i| (i, second_index))),
                            &ind[parsing_index..],
                        );
                    } else {
                        if second_index.is_some() {
                            println!("Error at {}", line!());
                            return (Err(()), &ind[parsing_index..]);
                        }
                        // No need to check for it's validity cause WE are pushing only the number
                        // chars into it
                        let index = number_buffer.parse::<i32>().unwrap();
                        // No need to clear this, since there will be no more parsing

                        //Test if the index will be underflowed
                        if -offset >= index {
                            println!("Error at {}", line!());
                            return (Err(()), &ind[parsing_index..]);
                        }

                        if index + offset > self.lines.len() as i32 {
                            println!("Error at {}", line!());
                            return (Err(()), &ind[parsing_index..]);
                        }

                        return (
                            if first_index.is_some() {
                                if index + offset < first_index.unwrap() as i32 {
                                    println!("Index {index}, first index {:?}", first_index);
                                    println!("Error at {}", line!());
                                    Err(())
                                } else {
                                    Ok(Some((
                                        first_index.unwrap(),
                                        Some((index + offset) as usize),
                                    )))
                                }
                            } else {
                                Ok(Some(((index + offset) as usize, None)))
                            },
                            &ind[parsing_index..],
                        );
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
    let mut buffer = Buffer::default();
    buffer.cursor = 1;
    buffer.lines = (0..20).map(|i| i.to_string()).collect();

    let command = "1,10p";
    let (index, command) = buffer.parse_index(command);

    assert_eq!(index, Ok(Some((1, Some(10)))));
    assert_eq!(command, "p");

    let command = "0,10p";
    let (index, _) = buffer.parse_index(command);

    assert_eq!(index, Err(()));

    let command = "1,201p";
    let (index, _) = buffer.parse_index(command);

    assert_eq!(index, Err(()));

    let command = "0,201p";
    let (index, _) = buffer.parse_index(command);

    assert_eq!(index, Err(()));
}

#[test]
fn subtraction_test() {
    let mut buffer = Buffer::default();
    buffer.cursor = 1;
    buffer.lines = (0..20).map(|i| i.to_string()).collect();

    let command = "1,--10p";
    let (index, command) = buffer.parse_index(command);

    assert_eq!(index, Ok(Some((1, Some(8)))));
    assert_eq!(command, "p");

    let command = "--3,10p";
    let (index, command) = buffer.parse_index(command);

    assert_eq!(index, Ok(Some((1, Some(10)))));
    assert_eq!(command, "p");

    let command = "--3,--10p";
    let (index, command) = buffer.parse_index(command);

    assert_eq!(index, Ok(Some((1, Some(8)))));
    assert_eq!(command, "p");

    let command = "--1,10p";
    let (index, _) = buffer.parse_index(command);

    assert_eq!(index, Err(()));

    let command = "1,--1p";
    let (index, _) = buffer.parse_index(command);

    assert_eq!(index, Err(()));

    let command = "--1,--1p";
    let (index, _) = buffer.parse_index(command);

    assert_eq!(index, Err(()));
}

#[test]
fn addition_test() {
    let mut buffer = Buffer::default();
    buffer.cursor = 1;
    buffer.lines = (0..20).map(|i| i.to_string()).collect();

    let command = "1,++10p";
    let (index, command) = buffer.parse_index(command);

    assert_eq!(index, Ok(Some((1, Some(12)))));
    assert_eq!(command, "p");

    let command = "++1,10p";
    let (index, command) = buffer.parse_index(command);

    assert_eq!(index, Ok(Some((3, Some(10)))));
    assert_eq!(command, "p");

    let command = "++1,++10p";
    let (index, command) = buffer.parse_index(command);

    assert_eq!(index, Ok(Some((3, Some(12)))));
    assert_eq!(command, "p");
}

#[test]
fn single_index_test() {
    let mut buffer = Buffer::default();
    buffer.cursor = 1;
    buffer.lines = (0..9).map(|i| i.to_string()).collect();

    let command = ",p";
    println!("Testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((1, Some(9)))));
    assert_eq!(command, "p");

    let command = ";p";
    println!("Testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((0, Some(9)))));
    assert_eq!(command, "p");

    let command = ";++p";
    println!("Testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Err(()));
    assert_eq!(command, "p");

    let command = ";12p";
    println!("Testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Err(()));
    assert_eq!(command, "p");

    let command = "++;p";
    println!("Testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Err(()));
    assert_eq!(command, "p");
}

#[test]
fn dollar_sign_test() {
    let mut buffer = Buffer::default();
    buffer.cursor = 1;
    buffer.lines = (0..9).map(|i| i.to_string()).collect();

    let command = "1,$p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((1, Some(9)))));
    assert_eq!(command, "p");

    let command = "$p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((9, None))));
    assert_eq!(command, "p");

    let command = "$,9p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((9, Some(9)))));
    assert_eq!(command, "p");

    let command = "$,$p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((9, Some(9)))));
    assert_eq!(command, "p");

    let command = "--$,9p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((7, Some(9)))));
    assert_eq!(command, "p");

    let command = "1,--$p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((1, Some(7)))));
    assert_eq!(command, "p");
    let command = "--$,--$p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((7, Some(7)))));
    assert_eq!(command, "p");

    let command = "$,1p";
    println!("testing {command}");
    let (index, _) = buffer.parse_index(command);
    assert_eq!(index, Err(()));

    let command = "+$,1p";
    println!("testing {command}");
    let (index, _) = buffer.parse_index(command);
    assert_eq!(index, Err(()));
}

#[test]
fn dot_test() {
    let mut buffer = Buffer::default();
    buffer.cursor = 1;
    buffer.lines = (0..9).map(|i| i.to_string()).collect();

    let command = "1,.p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((1, Some(1)))));
    assert_eq!(command, "p");

    let command = ".p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((1, None))));
    assert_eq!(command, "p");

    let command = ".,9p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((1, Some(9)))));
    assert_eq!(command, "p");

    let command = ".,.p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((1, Some(1)))));
    assert_eq!(command, "p");

    let command = "++.,9p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((3, Some(9)))));
    assert_eq!(command, "p");

    let command = "1,++.p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((1, Some(3)))));
    assert_eq!(command, "p");
    let command = "++.,++.p";
    println!("testing {command}");
    let (index, command) = buffer.parse_index(command);
    assert_eq!(index, Ok(Some((3, Some(3)))));
    assert_eq!(command, "p");

    let command = "9,.p";
    println!("testing {command}");
    let (index, _) = buffer.parse_index(command);
    assert_eq!(index, Err(()));

    let command = "-.,9p";
    println!("testing {command}");
    let (index, _) = buffer.parse_index(command);
    assert_eq!(index, Err(()));
}
