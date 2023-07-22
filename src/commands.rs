pub enum Operation {
    Quit,
    Error(&'static str),
    SetPrompt(String),
    Insert,
    Append,
    Write(String),
}
pub fn parse_command(command: &str) -> Operation {
    match command.chars().nth(0).unwrap_or('q') {
        'q' | 'Q' => Operation::Quit,
        'P' => {
            if command.len() == 2 {
                Operation::SetPrompt("*".into())
            } else {
                Operation::SetPrompt(command[1..].into())
            }
        }
        'i' => Operation::Insert,
        'a' => Operation::Append,
        'w' => Operation::Write(command[1..].into()),
        _ => Operation::Error("Unknown command"),
    }
}
