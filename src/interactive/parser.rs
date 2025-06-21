

use super::SessionMode;

pub enum InteractiveCommand {
    Quit,
    ChangeMode(SessionMode),
    Text(String),
}

pub fn parse_input(line: &str) -> Result<InteractiveCommand, String> {
    if line.starts_with('\\') {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        let command = parts[0];
        match command {
            "\\quit" => Ok(InteractiveCommand::Quit),
            "\\journal" => Ok(InteractiveCommand::ChangeMode(SessionMode::Journal)),
            "\\snippet" => Ok(InteractiveCommand::ChangeMode(SessionMode::Snippet)),
            "\\note" => Ok(InteractiveCommand::ChangeMode(SessionMode::Note)),
            _ => Err(format!("Unknown command: {}", command)),
        }
    } else {
        Ok(InteractiveCommand::Text(line.to_string()))
    }
}

