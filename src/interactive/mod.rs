
use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::db;

pub fn run(database: &db::Database) {
    let mut rl = Editor::<(), rustyline::history::DefaultHistory>::new().expect("Failed to create readline editor");
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line == "\\quit" {
                    break;
                }
                println!("You typed: {}", line);
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
