


use rusqlite::{Connection, ErrorCode};
use std::fs;
use std::path::Path;

#[test]
fn test_database_initialization() {
    // Create a temporary database file
    let db_path = "test_db_init_test.db";
    if Path::new(db_path).exists() {
        fs::remove_file(db_path).unwrap();
    }

    // Create a new connection to the database
    let conn = Connection::open(Path::new(db_path)).expect("Failed to open database");

    // Check that dev_logs table doesn't exist initially
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='dev_logs'").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 0, "dev_logs table should not exist initially");

    // Read the entire content of sql/init.sql file
    let init_sql_path = "sql/init.sql";
    let init_sql_content = fs::read_to_string(init_sql_path)
        .expect(&format!("Failed to read {}", init_sql_path));

    // Execute the SQL batch
    conn.execute_batch(&init_sql_content)
        .expect("Failed to execute batch from init.sql");

    // Check that dev_logs table exists after initialization
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='dev_logs'").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 1, "dev_logs table should exist after initialization");

    // Test inserting data into dev_logs
    conn.execute("INSERT INTO dev_logs (entry, tags) VALUES ('Test entry', 'test')", [])
        .expect("Failed to insert test data");

    // Verify the data was inserted
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM dev_logs").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    assert_eq!(count, 1, "Should have one entry in dev_logs table");

    // Clean up
    fs::remove_file(db_path).unwrap();
}


