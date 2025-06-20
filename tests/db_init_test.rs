// It's conventional to name integration test files with a _test suffix.
// e.g., database_test.rs or db_test.rs

// This is the correct way to import your library's modules for use in tests.
use cli_notes::db;
use std::fs;
use std::path::Path;

/// A simple guard struct to ensure the test database file is cleaned up
/// automatically when the test function goes out of scope.
struct TestDbGuard<'a> {
    path: &'a str,
}

impl<'a> Drop for TestDbGuard<'a> {
    fn drop(&mut self) {
        if Path::new(self.path).exists() {
            fs::remove_file(self.path).expect("Failed to clean up test database file.");
        }
    }
}

#[test]
fn test_database_lifecycle() {
    // --- SETUP ---
    // Define a unique path for our test database and ensure it's cleaned up.
    let db_path = "test_suite_main.db";
    let _guard = TestDbGuard { path: db_path }; // Cleanup runs when _guard is dropped.

    // --- ACT & ASSERT: Part 1 - Creation ---
    // The test begins here. We expect `new` to successfully create the database file.
    let db = db::Database::new(db_path).expect("Database::new should succeed.");
    assert!(Path::new(db_path).exists(), "Database file should be created.");

    // --- ACT & ASSERT: Part 2 - Initialization ---
    // Now, test the initialization. We expect it to run without errors.
    // The `initialize` method itself is responsible for creating and verifying
    // all the necessary tables and triggers. Our test just needs to confirm
    // that the method reports success.
    db.initialize()
        .expect("Database::initialize should succeed.");

    // --- ACT & ASSERT: Part 3 - Data Interaction ---
    // Now that the database is initialized, we can test a simple data insertion
    // to ensure the tables are usable.
    let insert_result = db.conn().execute(
        "INSERT INTO journal_entries (entry, tags) VALUES (?1, ?2)",
        ["This is a test entry from an integration test.", "test, rust"],
    );
    println!("Insert result: {:?}", insert_result); 
    assert!(
        insert_result.is_ok(),
        "Should be able to insert into journal_entries after initialization."
    );
    assert_eq!(
        insert_result.unwrap(),
        1,
        "The insertion should affect exactly one row."
    );

    // Verify the data was actually inserted.
    let entry_count: i64 = db
        .conn()
        .query_row("SELECT COUNT(*) FROM journal_entries", [], |row| row.get(0))
        .expect("Should be able to query journal_entries.");

    assert_eq!(
        entry_count, 1,
        "journal_entries table should contain exactly one entry."
    );

    // The _guard will now go out of scope, and its Drop implementation will delete the file.
}