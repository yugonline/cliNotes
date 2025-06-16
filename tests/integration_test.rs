


use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;
use cli_notes::db;

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
fn test_database_initialization() {
    // Create a temporary database file
    let db_path = "test_db_init_test.db";
    let _guard = TestDbGuard { path: db_path }; // Cleanup runs when _guard is dropped.

    // Create a new connection to the database
    let db = db::Database::new(db_path).expect("Database::new should succeed.");
    assert!(Path::new(db_path).exists(), "Database file should be created.");

    // Check that dev_logs table doesn't exist initially
    let mut stmt = db.conn().prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='dev_logs'").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 0, "dev_logs table should not exist initially");

    // Initialize the database
    db.initialize().expect("Failed to initialize database");

    // Check that dev_logs table exists after initialization
    let mut stmt = db.conn().prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='dev_logs'").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 1, "dev_logs table should exist after initialization");

    // Test inserting data into dev_logs
    db.conn().execute("INSERT INTO dev_logs (entry, tags) VALUES ('Test entry', 'test')", []).unwrap();

    // Verify the data was inserted
    let mut stmt = db.conn().prepare("SELECT COUNT(*) FROM dev_logs").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 1, "Should have one entry in dev_logs table");

    // Clean up
    fs::remove_file(db_path).unwrap();
}

