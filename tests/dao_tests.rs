use rusqlite::Connection;
use cli_notes::dao::{self, *};
use cli_notes::models::{CodeSnippet, JournalEntry};

#[test]
fn test_create_and_read_code_snippet() {
    // Create an in-memory database
    let conn = Connection::open_in_memory().unwrap();

    // Create the necessary tables
    conn.execute(
        "CREATE TABLE languages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).unwrap();

    conn.execute(
        "CREATE TABLE code_snippets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            full_code TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            language_id INTEGER,
            FOREIGN KEY(language_id) REFERENCES languages(id)
        )",
        [],
    ).unwrap();

    // Insert a test language
    conn.execute(
        "INSERT INTO languages (name) VALUES ('rust')",
        [],
    ).unwrap();

    // Create a code snippet
    let snippet = CodeSnippet {
        id: 0,
        full_code: String::from("fn main() { println!(\"Hello, world!\"); }"),
        created_at: chrono::Local::now(),
        updated_at: chrono::Local::now(),
        language_id: 1,
    };

    // Create the code snippet in the database
    let snippet_id = dao::create_code_snippet(&conn, &snippet, "rust").unwrap();

    // Read the code snippet back from the database
    let mut read_snippet = dao::read_code_snippet(&conn, snippet_id).unwrap().unwrap();

    // Verify that the code snippet was created correctly - account for preprocessed code
    assert!(read_snippet.full_code.starts_with("'''\n"));
    assert!(read_snippet.full_code.ends_with("\n'''"));
    assert!(read_snippet.full_code.contains("fn main() { println!(\"Hello, world!\"); }"));

    // Verify language_id
    assert_eq!(read_snippet.language_id, 1);

    // Update the code snippet
    read_snippet.full_code.push_str("\n// Updated code");
    dao::update_code_snippet(&conn, &read_snippet).unwrap();

    // Read the updated code snippet
    let updated_snippet = dao::read_code_snippet(&conn, snippet_id).unwrap().unwrap();
    assert!(updated_snippet.full_code.ends_with("\n// Updated code\n'''"));

    // Delete the code snippet
    dao::delete_code_snippet(&conn, snippet_id).unwrap();

    // Verify that the code snippet was deleted
    let deleted_snippet = dao::read_code_snippet(&conn, snippet_id).unwrap();
    assert!(deleted_snippet.is_none());
}

#[test]
fn test_create_and_read_journal_entry() {
    // Create an in-memory database
    let conn = Connection::open_in_memory().unwrap();

    // Create the necessary tables
    conn.execute(
        "CREATE TABLE journal_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            entry TEXT NOT NULL,
            date DATETIME DEFAULT CURRENT_TIMESTAMP,
            tags TEXT,
            sentiment TEXT,
            ai_tags TEXT
        )",
        [],
    ).unwrap();

    // Create a journal entry
    let mut journal_entry = JournalEntry::new(
        String::from("Today I worked on the Rust DAO module and I'm excited about the progress!"),
        Some(String::from("rust, testing")),
    );

    // Finalize the journal entry (sets default values)
    journal_entry.finalize();

    // Create the journal entry in the database
    let entry_id = dao::create_journal_entry(&conn, &journal_entry).unwrap();

    // Read the journal entry back from the database
    let read_entry = dao::read_journal_entry(&conn, entry_id).unwrap().unwrap();

    // Verify that the journal entry was created correctly - account for preprocessed code
    assert!(read_entry.entry.starts_with("'''\n"));
    assert!(read_entry.entry.ends_with("\n'''"));
    assert!(read_entry.entry.contains("Today I worked on the Rust DAO module and I''m excited about the progress!"));

    // Check tags if present - now stored directly without preprocessing
    if let Some(tags) = read_entry.tags {
        assert!(tags == "rust, testing");
    }

    // Check AI-generated sentiment (should be positive due to "excited")
    if let Some(sentiment) = read_entry.sentiment {
        assert!(sentiment == "positive");
    }

    // Check AI-generated tags (should include "rust" from the content)
    if let Some(ai_tags) = read_entry.ai_tags {
        println!("AI tags: {}", ai_tags);
        assert!(ai_tags.contains("rust"));
    } else {
        println!("No AI tags found");
        panic!("Expected AI tags to be present");
    }
}

#[test]
fn test_language_functions() {
    // Create an in-memory database
    let conn = Connection::open_in_memory().unwrap();

    // Create the languages table
    conn.execute(
        "CREATE TABLE languages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).unwrap();

    // Insert a test language
    conn.execute(
        "INSERT INTO languages (name) VALUES ('javascript')",
        [],
    ).unwrap();

    // Test get_language_from_id
    let lang_name = dao::get_language_from_id(&conn, 1).unwrap();
    assert_eq!(lang_name, "javascript");

    // Test get_language_id_from_name
    let lang_id = dao::get_language_id_from_name(&conn, "javascript").unwrap();
    assert_eq!(lang_id, 1);

    // Test language_exists
    let exists = dao::language_exists(&conn, "javascript").unwrap();
    assert!(exists);

    // Test language_exists with non-existent language
    let exists = dao::language_exists(&conn, "nonexistent").unwrap();
    assert!(!exists);
}
