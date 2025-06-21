use cli_notes::db;
use std::path::PathBuf;
use cli_notes::dao::{create_journal_entry, get_journal_entries_by_period, search_journal_entries, summarize_journal_entries};
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

#[derive(Subcommand, Debug)]
enum SnippetCommands {
    /// Add a new code snippet
    Add {
        /// The code content of the snippet
        #[arg(long)]
        code: String,
        /// The programming language of the snippet (e.g., rust, python)
        #[arg(long)]
        lang: String,
    },
    /// Show a specific code snippet by its ID
    Show {
        /// The ID of the snippet to show
        id: i64,
    },
}



fn main() {
    use std::env;

    if let Ok(path) = env::current_dir() {
        println!("The current directory is {}", path.display());
    }

    // Replace the old db_path line with this block
    let db_path: PathBuf = match dirs::config_dir() {
        Some(mut path) => {
            path.push("clinotes"); // Create a directory for our app
            if !path.exists() {
                std::fs::create_dir_all(&path).expect("Failed to create config directory");
            }
            path.push("clinotes.db"); // The final DB file path
            path
        }
        None => {
            // Fallback for rare cases where config dir can't be found
            eprintln!("Warning: Could not find a config directory. Using current directory.");
            PathBuf::from("clidblocal.db")
        }
    };

    // Create a new database connection
    let database = match db::Database::new(db_path.to_str().unwrap()) {
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
                    match create_journal_entry(database.conn(), &journal_entry) {
                        Ok(id) => {
                            println!("✅ Journal entry created successfully with ID: {}", id);
                            println!("🤖 AI analysis completed - sentiment and tags automatically generated!");
                        }
                        Err(e) => println!("❌ Error creating journal entry: {}", e),
                    }
                }
                JournalCommands::Summarize { period } => {
                    match get_journal_entries_by_period(database.conn(), &period) {
                        Ok(entries) => {
                            if entries.is_empty() {
                                println!("📝 No journal entries found for the {} period.", period);
                            } else {
                                println!("📊 AI Summary for the past {}:", period);
                                println!("Found {} entries", entries.len());
                                
                                let summary = summarize_journal_entries(&entries);
                                
                                println!("\n🎭 Sentiment Analysis:");
                                println!("  Positive: {} entries", summary.positive_count);
                                println!("  Negative: {} entries", summary.negative_count);
                                println!("  Neutral: {} entries", summary.neutral_count);
                                
                                println!("\n🏷️  Most common topics: {}", summary.common_topics);
                            }
                        }
                        Err(e) => println!("❌ Error retrieving entries: {}", e),
                    }
                }
                JournalCommands::Insights { query } => {
                    match search_journal_entries(database.conn(), &query) {
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
