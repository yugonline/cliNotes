use rusqlite::{Connection, ErrorCode, Result};
use std::fs;
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Creates a new database or opens an existing one
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(Path::new(db_path))?;
        Ok(Database { conn })
    }

    /// Initialize database by creating tables if they dont exist.
    pub fn initialize(&self) -> Result<(), rusqlite::Error> {
        // Check if dev_logs table exists
        if !self.check_existence("table", "dev_logs")? {
            // Read the entire content of sql/init.sql file and execute it as a batch
            let init_sql_path = "sql/init.sql";
            let init_sql_content = fs::read_to_string(init_sql_path)
                .map_err(|e| rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error {
                        code: ErrorCode::Unknown,
                        extended_code: 1,
                    },
                    Some(format!("Failed to read {}: {}", init_sql_path, e)),
                ))?;

            self.conn.execute_batch(&init_sql_content)
                .map_err(|e| rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error {
                        code: ErrorCode::Unknown,
                        extended_code: 1,
                    },
                    Some(format!("Failed to execute batch from {}: {}", init_sql_path, e)),
                ))?;
        }

        // Verify all required tables and triggers exist
        let required_tables = vec![
            "dev_logs",
            "languages",
            "code_snippets",
            "learning_notes",
            "snippets_used",
        ];
        let required_triggers = vec![
            "update_timestamp_after_update_code_snippets",
            "update_timestamp_after_update_languages",
            "update_timestamp_after_update_learning_notes",
        ];

        for table in &required_tables {
            if !self.check_existence("table", table)? {
                return Err(rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error {
                        code: ErrorCode::Unknown,
                        extended_code: 1,
                    },
                    Some(format!(
                        "Table {} doesn't exist. Please run 'sqlite3 your_database_file.db < sql/init.sql'",
                        table
                    )),
                ));
            }
        }

        for trigger in &required_triggers {
            if !self.check_existence("trigger", trigger)? {
                return Err(rusqlite::Error::SqliteFailure(
                    rusqlite::ffi::Error {
                        code: ErrorCode::Unknown,
                        extended_code: 1,
                    },
                    Some(format!(
                        "Trigger {} doesn't exist. Please run 'sqlite3 your_database_file.db < sql/init.sql'",
                        trigger
                    )),
                ));
            }
        }

        Ok(())
    }

    fn check_existence(&self, kind: &str, name: &str) -> Result<bool, rusqlite::Error> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = ?1 AND name = ?2",
            &[kind, name],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }
}
