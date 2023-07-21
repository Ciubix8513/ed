pub enum Operation {
    Quit,
    Error(&'static str),
    SetPrompt(String),
    Insert,
    Append,
}
pub fn parse_command(command: &str) -> Operation {
    match command.chars().nth(0).unwrap_or_default() {
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
        _ => Operation::Error("Unknown command"),
    }
}
