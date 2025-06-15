


use rusqlite::{Connection};
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

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

        // Initialize the database
        let db = crate::db::Database { conn };
        db.initialize().expect("Failed to initialize database");

        // Check that dev_logs table exists after initialization
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='dev_logs'").unwrap();
        let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
        assert_eq!(count, 1, "dev_logs table should exist after initialization");

        // Test inserting data into dev_logs
        conn.execute("INSERT INTO dev_logs (entry, tags) VALUES ('Test entry', 'test')", []).unwrap();

        // Verify the data was inserted
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM dev_logs").unwrap();
        let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
        assert_eq!(count, 1, "Should have one entry in dev_logs table");

        // Clean up
        fs::remove_file(db_path).unwrap();
    }
}


