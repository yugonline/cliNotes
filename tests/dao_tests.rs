// yugonline-clinotes/tests/dao_tests.rs

use cli_notes::dao;
use cli_notes::db;
use cli_notes::models::{CodeSnippet, JournalEntry};

/// Creates an in-memory SQLite database and returns an initialized `Database` instance.
fn setup_test_db() -> db::Database {
    // Open an in-memory database connection.
    let conn = rusqlite::Connection::open_in_memory()
        .expect("Failed to create in-memory database for testing.");

    // The Database struct now takes ownership of the connection.
    let db_instance = db::Database { conn };

    // Initialize the schema using the method on our instance.
    db_instance
        .initialize()
        .expect("Database initialization failed in test setup.");

    // Return the whole, ready-to-use instance.
    db_instance
}

#[test]
fn test_create_and_read_code_snippet() {
    // Use the in-memory database. All tables are already created by the helper.
    let db = setup_test_db();

    // The 'rust' language is already inserted by init.sql. No need to do it here.

    // Get the ID for 'rust' which was pre-populated by the init script.
    let rust_lang_id = dao::get_language_id_from_name(db.conn(), "rust").unwrap();

    // Create a code snippet
    let snippet = CodeSnippet {
        id: 0,
        full_code: String::from("fn main() { println!(\"Hello, world!\"); }"),
        created_at: chrono::Local::now(),
        updated_at: chrono::Local::now(),
        language_id: rust_lang_id, // Use the dynamically fetched ID
    };

    // Create the code snippet in the database
    let snippet_id = dao::create_code_snippet(db.conn(), &snippet, "rust").unwrap();

    // Read the code snippet back from the database
    let mut read_snippet = dao::read_code_snippet(db.conn(), snippet_id).unwrap().unwrap();

    // Verify that the code snippet was created correctly - account for preprocessed code
    assert!(read_snippet.full_code.starts_with("'''\n"));
    assert!(read_snippet.full_code.ends_with("\n'''"));
    assert!(read_snippet.full_code.contains("fn main() { println!(\"Hello, world!\"); }"));

    // Verify language_id
    assert_eq!(read_snippet.language_id, rust_lang_id);

    // Update the code snippet
    read_snippet.full_code.push_str("\n// Updated code");
    // NOTE: This assumes the `update_code_snippet` bug I mentioned before is also fixed.
    dao::update_code_snippet(db.conn(), &read_snippet).unwrap();

    // Read the updated code snippet
    let updated_snippet = dao::read_code_snippet(db.conn(), snippet_id).unwrap().unwrap();
    assert!(updated_snippet.full_code.ends_with("\n// Updated code\n'''"));

    // Delete the code snippet
    dao::delete_code_snippet(db.conn(), snippet_id).unwrap();

    // Verify that the code snippet was deleted
    let deleted_snippet = dao::read_code_snippet(db.conn(), snippet_id).unwrap();
    assert!(deleted_snippet.is_none());
}

#[test]
fn test_create_and_read_journal_entry() {
    // Create an in-memory database. `journal_entries` table is already created.
    let db = setup_test_db();

    // Create a journal entry
    let journal_entry = JournalEntry::new(
        String::from("Today I worked on the Rust DAO module and I'm excited about the progress!"),
        Some(String::from("rust, testing")),
    );

    // Create the journal entry in the database
    let entry_id = dao::create_journal_entry(db.conn(), &journal_entry).unwrap();

    // Read the journal entry back from the database
    let read_entry = dao::read_journal_entry(db.conn(), entry_id).unwrap().unwrap();

    // --- Assertions ---
    assert_eq!(
        read_entry.entry,
        "Today I worked on the Rust DAO module and I'm excited about the progress!"
    );

    if let Some(tags) = read_entry.tags {
        assert_eq!(tags, "rust, testing");
    }

    if let Some(sentiment) = read_entry.sentiment {
        assert_eq!(sentiment, "positive");
    }

    if let Some(ai_tags) = read_entry.ai_tags {
        println!("AI tags: {}", ai_tags);
        assert!(ai_tags.contains("rust"));
    } else {
        panic!("Expected AI tags to be present");
    }
}

#[test]
fn test_language_functions() {
    // Create an in-memory database. `languages` table is already created and populated.
    let db = setup_test_db();

    // The 'javascript' language is already inserted by init.sql.

    // Get the ID for 'javascript' to use in our test
    let js_lang_id = dao::get_language_id_from_name(db.conn(), "js").unwrap();

    // Test get_language_from_id
    let lang_name = dao::get_language_from_id(db.conn(), js_lang_id).unwrap();
    assert_eq!(lang_name, "js");

    // Test get_language_id_from_name
    let lang_id = dao::get_language_id_from_name(db.conn(), "js").unwrap();
    assert_eq!(lang_id, js_lang_id);

    // Test language_exists
    let exists = dao::language_exists(db.conn(), "js").unwrap();
    assert!(exists);

    // Test language_exists with non-existent language
    let exists_not = dao::language_exists(db.conn(), "nonexistent").unwrap();
    assert!(!exists_not);
}