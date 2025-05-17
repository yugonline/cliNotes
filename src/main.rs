mod dao;
mod db;
mod models;
mod schema;


use clap::{Parser, Subcommand};
use dao::create_dev_log;
use models::{DevLogArgs, NewDevLog};

#[derive(Parser)]
#[command(version, author, about = "Manage your dev logs, learning notes, and code snippets")]
struct CliNotes {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new development log entry
    DevLog(DevLogArgs),
use crate::dao::create_dev_log;
use crate::models::DevLog;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    version = "0.1",
    author = "Your Name <youremail@example.com>",
    about = "Manage your dev logs, learning notes, and code snippets"
)]
struct CliNotes {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a development log entry
    DevLog(DevLog),
    // Additional subcommands can be added here
}

fn main() {
    let args = CliNotes::parse();
    let mut db = db::Database::new("clidblocal.db").expect("failed to connect DB");
    db.initialize().expect("failed to init DB");

    match args.command {
        Commands::DevLog(devlog_args) => {
            let new_log = NewDevLog::from(&devlog_args);
            create_dev_log(db.get_connection(), &new_log).expect("failed to insert");
            println!("Added dev log");
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


    let opts: CliNotes = CliNotes::parse();
    match opts.command {
        Some(Commands::DevLog(mut devlog)) => {
            devlog.finalize();
            create_dev_log(database.get_connection(), &devlog).unwrap();
        }
        None => {
            println!("---------------------------------------------------");
            println!(" ██████ ██      ██ ███    ██  ██████  ████████ ███████ ███████ ");
            println!("██      ██      ██ ████   ██ ██    ██    ██    ██      ██      ");
            println!("██      ██      ██ ██ ██  ██ ██    ██    ██    █████   ███████ ");
            println!("██      ██      ██ ██  ██ ██ ██    ██    ██    ██           ██ ");
            println!(" ██████ ███████ ██ ██   ████  ██████     ██    ███████ ███████ ");
            println!("                                                                ");
            println!("                                                                ");
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
    }
}
