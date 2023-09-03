mod dao;
mod db;
mod models;

extern crate clap;

use crate::dao::create_dev_log;
use crate::models::DevLog;
use clap::{App, Arg, SubCommand};

fn main() {
    use std::env;

    if let Ok(path) = env::current_dir() {
        println!("The current directory is {}", path.display());
    }

    // Path to the SQLite database file
    let db_path = "clidblocal.db";

    // Create a new database connection
    let database = db::Database::new(db_path).expect("Failed to connect to the database ");

    //Initialize the database ( create tables if they don't exist)
    database
        .initialize()
        .expect("Failed to initialize the database");

    let matches = App::new("CliNotes")
        .version("0.1")
        .author("Your Name <youremail@example.com>")
        .about("Manage your dev logs, learning notes, and code snippets")
        .subcommand(
            SubCommand::with_name("devlog")
                .about("Add a new dev log entry")
                .arg(
                    Arg::with_name("ENTRY")
                        .help("The content of the dev log entry")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("TAGS")
                        .help("Relevant tags for your logs")
                        .required(false)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("codesnip")
                .about("Add a new code snippet")
                .arg(
                    Arg::with_name("CODE")
                        .help("The content of the code snippet")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("LANG")
                        .help("The language of the code snippet")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("learning_notes")
                .about("Add a new learning note")
                .arg(
                    Arg::with_name("FILE")
                        .help("Path to the markdown file")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("devlog", Some(devlog_matches)) => {
            let entry = devlog_matches.value_of("ENTRY").unwrap();
            let tags = devlog_matches.value_of("TAGS").unwrap();
            let log = DevLog::new(entry.to_string(), Some(tags.to_string()));
            create_dev_log(database.get_connection(), &log).unwrap();
            println!("Added dev log: {}", entry);
        }
        ("codesnip", Some(codesnip_matches)) => {
            let code = codesnip_matches.value_of("CODE").unwrap();
            // Call your function to handle code snippet addition
            println!("Added code snippet: {}", code);
        }
        ("learning_notes", Some(learning_notes_matches)) => {
            let file_path = learning_notes_matches.value_of("FILE").unwrap();
            // Call your function to handle learning note addition
            println!("Added learning note from file: {}", file_path);
        }
        _ => {
            println!("---------------------------------------------------");
            println!("   ______ _       _             ");
            println!("  / _____) |     | |       _    _ _ _         ");
            println!(" | /     | | ___ | | ____ _| |_| |_| |_  ___ ");
            println!(" | |     | |/ _ \\| |/ _  ) |_|  _)  _) |/ _ \\    ");
            println!(" | \\_____| | |_| ( ( (/ /| | |_| |_| | |_| |");
            println!("  \\______)_|\\___/ \\_/ \\_)(_|\\___)___/ \\___/ ");
            println!("");
            println!("Welcome to CliNotes!");
            println!("");
            println!("[1] View Dev Logs (Latest 3 entries)");
            println!("[2] View Learning Notes (Latest 3 entries)");
            println!("[3] View Code Snippets (Last 5 entries)");
            println!("[4] Add new Code Snippet");
            println!("[5] Exit");
            println!("---------------------------------------------------");
        }
    }
}
