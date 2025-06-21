
use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::db;
use crate::output;

use crate::dao;
use crate::models::{JournalEntry};


mod parser;



#[derive(Debug, PartialEq)]
pub enum SessionMode {
    Journal,
    Snippet,
    Note,
    Search,
    Help,
}


impl std::fmt::Display for SessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionMode::Journal => write!(f, "journal"),
            SessionMode::Snippet => write!(f, "snippet"),
            SessionMode::Note => write!(f, "note"),
            SessionMode::Search => write!(f, "search"),
            SessionMode::Help => write!(f, "help"),
        }
    }
}



impl Default for SessionMode {
    fn default() -> Self {
        SessionMode::Journal
    }
}

#[derive(Debug, Default)]
pub struct SessionState {
    pub mode: SessionMode,
    // Add other session-related data here as needed
}


pub fn run(database: &db::Database) {
    let mut rl = Editor::<(), rustyline::history::DefaultHistory>::new().expect("Failed to create readline editor");
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut state = SessionState::default();

    loop {
        let prompt = format!("({}) >> ", state.mode);
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match parser::parse_input(&line, &state) {
                    Ok(command) => {
                        match command {
                            parser::InteractiveCommand::Quit => {
                                break;
                            },
                            parser::InteractiveCommand::ChangeMode(new_mode) => {
                                state.mode = new_mode;
                                output::display_success(&format!("Switched to {} mode.", state.mode));
                            },
                            parser::InteractiveCommand::AddJournal(content) => {
                                let journal_entry = JournalEntry::new(content, None); // Assuming no tags for now
                                match dao::create_journal_entry(database.conn(), &journal_entry) {
                                    Ok(id) => output::display_success(&format!("Journal entry created successfully with ID: {}", id)),
                                    Err(e) => eprintln!("❌ Error creating journal entry: {}", e),
                                }
                            },
                            parser::InteractiveCommand::Text(text) => {
                                println!("Command not yet implemented: {}", text);
                            },
                        }
                    },
                    Err(e) => {
                        println!("Error parsing command: {}", e);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").expect("Failed to save history.");
}
