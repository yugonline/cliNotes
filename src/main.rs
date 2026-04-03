use clap::{Parser, Subcommand};
use cli_notes::dao;
use cli_notes::dao::{
    create_code_snippet, create_journal_entry, create_learning_note, create_mood_entry,
    get_journal_entries_by_period, get_mood_entries, read_code_snippet, search_journal_entries,
    summarize_journal_entries,
};
use cli_notes::db;
use cli_notes::interactive;
use cli_notes::models::{CodeSnippet, JournalEntry};
use std::path::PathBuf;

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
    /// Code snippet operations
    Snippet {
        #[command(subcommand)]
        command: SnippetCommands,
    },
    /// Learning note operations
    Note {
        #[command(subcommand)]
        command: NoteCommands,
    },
    /// Mood tracking operations
    Mood {
        #[command(subcommand)]
        command: MoodCommands,
    },
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

#[derive(Subcommand, Debug)]
enum NoteCommands {
    /// Add a new learning note by its file path
    Add {
        /// The path to the note file
        path: String,
    },
    /// Show a specific learning note's details by its ID
    Show {
        /// The ID of the note to show
        id: i64,
    },
}

#[derive(Subcommand, Debug)]
enum MoodCommands {
    /// Log your current mood
    Log {
        /// The mood you are feeling (e.g., Happy, Focused, Tired)
        mood: String,
        /// Optional note about why you feel this way
        #[arg(long)]
        note: Option<String>,
    },
    /// View recent mood history
    History {},
}

fn handle_one_off_command(command: Commands, database: &db::Database) {
    match command {
        Commands::Journal { command } => match command {
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
                                println!(
                                    "{}. [{}] {}",
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
        },
        Commands::Snippet { command } => match command {
            SnippetCommands::Add { code, lang } => {
                let new_snippet = CodeSnippet {
                    id: 0,
                    full_code: code,
                    created_at: chrono::Local::now(),
                    updated_at: chrono::Local::now(),
                    language_id: 0,
                };
                match create_code_snippet(database.conn(), &new_snippet, &lang) {
                    Ok(id) => println!("✅ Snippet created successfully with ID: {}", id),
                    Err(e) => eprintln!("❌ Error creating snippet: {}", e),
                }
            }
            SnippetCommands::Show { id } => match read_code_snippet(database.conn(), id) {
                Ok(Some(snippet)) => {
                    println!("--- Snippet ID: {} ---", snippet.id);
                    println!("Language ID: {}", snippet.language_id);
                    println!("Created At: {}", snippet.created_at);
                    println!("---\n{} \n---", snippet.full_code);
                }
                Ok(None) => println!("🔍 Snippet with ID {} not found.", id),
                Err(e) => eprintln!("❌ Error reading snippet: {}", e),
            },
        },
        Commands::Note { command } => match command {
            NoteCommands::Add { path } => match create_learning_note(database.conn(), &path) {
                Ok(id) => println!("✅ Note linked successfully with ID: {}", id),
                Err(e) => eprintln!("❌ Error linking note: {}", e),
            },
            NoteCommands::Show { id } => match dao::read_learning_note(database.conn(), id) {
                Ok(Some(note)) => {
                    println!("Note ID: {}", note.id);
                    println!("File Path: {}", note.file_path);
                    println!("File Name: {}", note.file_name);
                    println!("Created At: {}", note.created_at);
                    println!("Updated At: {}", note.updated_at);
                }
                Ok(None) => println!("Note with ID {} not found.", id),
                Err(e) => eprintln!("Error reading note: {}", e),
            },
        },
        Commands::Mood { command } => match command {
            MoodCommands::Log { mood, note } => {
                match create_mood_entry(database.conn(), &mood, note.as_deref()) {
                    Ok(id) => println!("✅ Mood logged successfully with ID: {}", id),
                    Err(e) => eprintln!("❌ Error logging mood: {}", e),
                }
            }
            MoodCommands::History {} => match get_mood_entries(database.conn()) {
                Ok(entries) => {
                    if entries.is_empty() {
                        println!("📝 No mood entries found.");
                    } else {
                        println!("🌈 Mood History:");
                        for entry in entries.iter().take(10) {
                            println!(
                                "[{}]: {} - {}",
                                entry.date.format("%Y-%m-%d %H:%M"),
                                entry.mood,
                                entry.note.as_deref().unwrap_or("No note")
                            );
                        }
                    }
                }
                Err(e) => eprintln!("❌ Error retrieving mood history: {}", e),
            },
        },
    }
}

fn main() {
    let db_path: PathBuf = match dirs::config_dir() {
        Some(mut path) => {
            path.push("clinotes");
            if !path.exists() {
                std::fs::create_dir_all(&path).expect("Failed to create config directory");
            }
            path.push("clinotes.db");
            path
        }
        None => PathBuf::from("clinotes.db"),
    };

    let database = match db::Database::new(db_path.to_str().unwrap()) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("❌ Error connecting to the database: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = database.initialize() {
        eprintln!("❌ Error initializing database: {}", e);
        std::process::exit(1);
    }

    let opts: CliNotes = CliNotes::parse();
    match opts.command {
        Some(command) => handle_one_off_command(command, &database),
        None => interactive::run(&database),
    }
}
