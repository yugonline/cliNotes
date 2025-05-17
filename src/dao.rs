use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::{NewDevLog};
use crate::schema::dev_logs;
use crate::models::{CodeSnippet, DevLog};

fn preprocess_code(code: &str, language: &str) -> String {
    match language {
        "rust" | "ts" | "js" => {
            let escaped_code = code.replace("'", "''");
            format!("'''\n{}\n'''", escaped_code)
        }
        _ => panic!("Unsupported language."),
    }
}


/// Insert a new dev log into the database using Diesel
pub fn create_dev_log(conn: &mut SqliteConnection, dev_log: &NewDevLog) -> QueryResult<usize> {
    diesel::insert_into(dev_logs::table)
        .values(dev_log)
        .execute(conn)
  
pub fn read_dev_log(conn: &Connection, dev_log_id: i64) -> Result<Option<DevLog>, rusqlite::Error> {
    conn.query_row(
        "SELECT id, entry, date, tags FROM dev_logs WHERE id = ?1",
        params![dev_log_id],
        |row| {
            Ok(DevLog {
                id: row.get(0)?,
                entry: row.get(1)?,
                date: row.get(2)?,
                tags: row.get(3)?,
            })
        },
    )
    .optional()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_code() {
        let raw_code = r#"console.log("Hello, it's me!");"#;
        let processed_code = preprocess_code(raw_code, "js");
        assert_eq!(processed_code, "'''\nconsole.log(\"Hello, it''s me!\");\n'''");
    }

    #[test]
    fn test_rust_preprocess_code() {
        let raw_code = r#"fn main() {\n    println!(\"Hello\");\n}"#;
        let expected_code = "'''\nfn main() {\n    println!(\"Hello\");\n}\n'''";
        let processed_code = preprocess_code(raw_code, "rust");
        assert_eq!(processed_code, expected_code);
    }
}
