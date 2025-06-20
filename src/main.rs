use cli_notes::db;
use cli_notes::dao::{create_journal_entry, get_journal_entries_by_period, search_journal_entries};
use cli_notes::models::JournalEntry;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    version = "0.1",
    author = "Your Name <youremail@example.com>",
    about = "AI-Powered Journaling App - Manage your journal entries, learning notes, and code snippets with intelligent insights"
)]
struct CliNotes {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// AI-powered journal operations
    Journal {
        #[command(subcommand)]
        command: JournalCommands,
    },
    // Additional subcommands can be added here
}

#[derive(Subcommand, Debug)]
enum JournalCommands {
    /// Add a new journal entry with AI analysis
    Add {
        /// The journal entry content
        entry: String,
        /// Optional tags for the entry
        #[arg(long)]
        tags: Option<String>,
    },
    /// Generate AI summary for a specific time period
    Summarize {
        /// Time period (week, month, year)
        #[arg(long, default_value = "week")]
        period: String,
    },
    /// Ask AI questions about your journal entries
    Insights {
        /// Your question about the journal entries
        query: String,
    },
}


fn main() {
    use std::env;

    if let Ok(path) = env::current_dir() {
        println!("The current directory is {}", path.display());
    }

    // Path to the SQLite database file
    let db_path = "clidblocal.db";

    // Create a new database connection
    let database = match db::Database::new(db_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("❌ Error connecting to the database: {}", e);
            std::process::exit(1);
        }
    };

    //Initialize the database ( create tables if they don't exist)
    if let Err(e) = database.initialize() {
        eprintln!("❌ Error initializing database: {}", e);
        std::process::exit(1);
    }


    let opts: CliNotes = CliNotes::parse();
    match opts.command {
        Some(Commands::Journal { command }) => {
            match command {
                JournalCommands::Add { entry, tags } => {
                    let journal_entry = JournalEntry::new(entry, tags);
                    match create_journal_entry(database.get_connection(), &journal_entry) {
                        Ok(id) => {
                            println!("✅ Journal entry created successfully with ID: {}", id);
                            println!("🤖 AI analysis completed - sentiment and tags automatically generated!");
                        }
                        Err(e) => println!("❌ Error creating journal entry: {}", e),
                    }
                }
                JournalCommands::Summarize { period } => {
                    match get_journal_entries_by_period(database.get_connection(), &period) {
                        Ok(entries) => {
                            if entries.is_empty() {
                                println!("📝 No journal entries found for the {} period.", period);
                            } else {
                                println!("📊 AI Summary for the past {}:", period);
                                println!("Found {} entries", entries.len());
                                
                                // Simple AI summary generation
                                let mut positive_count = 0;
                                let mut negative_count = 0;
                                let mut neutral_count = 0;
                                let mut all_ai_tags = Vec::new();
                                
                                for entry in &entries {
                                    match entry.sentiment.as_deref() {
                                        Some("positive") => positive_count += 1,
                                        Some("negative") => negative_count += 1,
                                        _ => neutral_count += 1,
                                    }
                                    
                                    if let Some(ai_tags) = &entry.ai_tags {
                                        all_ai_tags.extend(ai_tags.split(',').map(|s| s.trim()));
                                    }
                                }
                                
                                println!("\n🎭 Sentiment Analysis:");
                                println!("  Positive: {} entries", positive_count);
                                println!("  Negative: {} entries", negative_count);
                                println!("  Neutral: {} entries", neutral_count);
                                
                                println!("\n🏷️  Most common topics: {}", 
                                    all_ai_tags.into_iter().collect::<std::collections::HashSet<_>>()
                                        .into_iter().take(5).collect::<Vec<_>>().join(", "));
                            }
                        }
                        Err(e) => println!("❌ Error retrieving entries: {}", e),
                    }
                }
                JournalCommands::Insights { query } => {
                    match search_journal_entries(database.get_connection(), &query) {
                        Ok(entries) => {
                            if entries.is_empty() {
                                println!("🔍 No entries found matching your query: '{}'", query);
                            } else {
                                println!("🧠 AI Insights for query: '{}'", query);
                                println!("Found {} relevant entries:\n", entries.len());
                                
                                for (i, entry) in entries.iter().take(3).enumerate() {
                                    println!("{}. [{}] {}", 
                                        i + 1, 
                                        entry.date.format("%Y-%m-%d"),
                                        entry.entry.chars().take(100).collect::<String>()
                                    );
                                    if let Some(sentiment) = &entry.sentiment {
                                        println!("   Sentiment: {}", sentiment);
                                    }
                                    if let Some(ai_tags) = &entry.ai_tags {
                                        println!("   AI Tags: {}", ai_tags);
                                    }
                                    println!();
                                }
                                
                                if entries.len() > 3 {
                                    println!("... and {} more entries", entries.len() - 3);
                                }
                            }
                        }
                        Err(e) => println!("❌ Error searching entries: {}", e),
                    }
                }
            }
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
            println!("Welcome to CliNotes - AI-Powered Journaling!");
            println!("");
            println!("🤖 AI Journal Features:");
            println!("[1] Add Journal Entry (with AI sentiment analysis & auto-tagging)");
            println!("[2] AI Summary (weekly/monthly insights)");
            println!("[3] AI Insights (ask questions about your entries)");
            println!("[4] View Learning Notes (Latest 3 entries)");
            println!("[5] View Code Snippets (Last 5 entries)");
            println!("[6] Add new Code Snippet");
            println!("[7] Exit");
            println!("");
            println!("💡 Try: 'cargo run -- journal add \"Today I learned Rust!\"'");
            println!("💡 Try: 'cargo run -- journal summarize --period week'");
            println!("💡 Try: 'cargo run -- journal insights \"How do I feel about coding?\"'");
            println!("---------------------------------------------------");
        }
    }
}
