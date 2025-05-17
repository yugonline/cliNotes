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
        }
    }
}
