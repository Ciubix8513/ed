pub enum Operation {
    Quit,
    Error,
    SetPrompt(String),
}
pub fn parse_command(command: &str) -> Operation {
    match command.chars().nth(0).unwrap_or_default() {
        'q' | 'Q' => Operation::Quit,
        'P' => {
            if command.len() == 2 {
                println!("Setting prompt to default");
                Operation::SetPrompt("*".into())
            } else {
                println!("Setting custom prompt , command len is {}", command.len());
                Operation::SetPrompt(command[1..].into())
            }
        }
        _ => Operation::Error,
    }
}
