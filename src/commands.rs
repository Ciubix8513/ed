pub enum Operation {
    Quit,
    Error(&'static str),
    TogglePrompt,
    Insert,
    Append,
    Write(String),
    ToggleVerbose,
}
pub fn parse_command(command: &str) -> Operation {
    match command.chars().next().unwrap_or(' ') {
        'q' | 'Q' => Operation::Quit,
        'P' => Operation::TogglePrompt,
        'i' => Operation::Insert,
        'a' => Operation::Append,
        'w' => Operation::Write(command[1..].into()),
        'H' => Operation::ToggleVerbose,
        _ => Operation::Error("Unknown command"),
    }
}
