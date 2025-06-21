

use super::SessionMode;

pub enum InteractiveCommand {
    Quit,
    ChangeMode(SessionMode),
    AddJournal(String), // New variant for adding journal entries
    Text(String),
}

pub fn parse_input(line: &str, state: &super::SessionState) -> Result<InteractiveCommand, String> {
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
        match state.mode {
            SessionMode::Journal => {
                if line.starts_with("add ") {
                    let content = line["add ".len()..].trim().to_string();
                    if content.is_empty() {
                        Err("Journal entry content cannot be empty.".to_string())
                    } else {
                        Ok(InteractiveCommand::AddJournal(content))
                    }
                } else {
                    Ok(InteractiveCommand::Text(line.to_string()))
                }
            },
            _ => Ok(InteractiveCommand::Text(line.to_string())),
        }
    }
}

