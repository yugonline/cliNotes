use crate::models::{CodeSnippet, DevLog};
use rusqlite::{params, Connection, OptionalExtension};
use std::fmt;

pub fn preprocess_code(code: &str, language: &str) -> Result<String, String> {
    match language {
        "rust" | "ts" | "js" => {
            let escaped_code = code.replace("'", "''");
            Ok(format!("'''\n{}\n'''", escaped_code))
        }
        _ => Err("Unsupported language.".to_string()),
    }
}

// Custom error type for our DAO operations
#[derive(Debug)]
pub enum DaoError {
    PreprocessingError(String),
    DatabaseError(rusqlite::Error),
}

impl fmt::Display for DaoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DaoError::PreprocessingError(msg) => write!(f, "Preprocessing error: {}", msg),
            DaoError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl From<rusqlite::Error> for DaoError {
    fn from(error: rusqlite::Error) -> Self {
        DaoError::DatabaseError(error)
    }
}

//Languages only READ operation available

pub fn get_language_from_id(conn: &Connection, language_id: i64) -> Result<String, DaoError> {
    conn.query_row(
        "SELECT name FROM languages WHERE id = ?",
        params![language_id],
        |row| Ok(row.get::<_, String>(0)?),
    ).map_err(DaoError::from)
}

pub fn get_language_id_from_name(conn: &Connection, lang_name: &str) -> Result<i64, DaoError> {
    conn.query_row(
        "SELECT id FROM languages WHERE name = ?",
        params![lang_name],
        |row| Ok(row.get::<_, i64>(0)?),
    ).map_err(DaoError::from)
}

pub fn language_exists(conn: &Connection, language_name: &str) -> Result<bool, DaoError> {
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM languages WHERE name = ?",
        params![language_name],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

// CRUD for Code Snippets
pub fn create_code_snippet(
    conn: &Connection,
    snippet: &CodeSnippet,
    lang_name: &str,
) -> Result<i64, DaoError> {
    let lang_id = get_language_id_from_name(conn, lang_name)?;

    let processed_code = preprocess_code(&snippet.full_code, lang_name)
        .map_err(DaoError::PreprocessingError)?;

    conn.execute(
        "INSERT INTO code_snippets (full_code, language_id) VALUES (?, ?)",
        params![processed_code, lang_id],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn read_code_snippet(
    conn: &Connection,
    snippet_id: i64,
) -> Result<Option<CodeSnippet>, DaoError> {
    conn.query_row(
        "SELECT id, full_code, created_at, updated_at, language_id FROM code_snippets WHERE id = ?1",
        params![snippet_id],
        |row| {
            Ok(CodeSnippet {
                id: row.get(0)?,
                full_code: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
                language_id: row.get(4)?,
            })
        },
    ).optional().map_err(DaoError::from)
}

pub fn update_code_snippet(
    conn: &Connection,
    snippet: &CodeSnippet,
) -> Result<(), DaoError> {
    let processed_code = preprocess_code(&snippet.full_code, "rust")
        .map_err(DaoError::PreprocessingError)?;

    conn.execute(
        "UPDATE code_snippets SET full_code = ?, language_id = ? WHERE id = ?",
        params![&processed_code, &snippet.language_id, &snippet.id],
    )?;
    Ok(())
}

pub fn delete_code_snippet(conn: &Connection, snippet_id: i64) -> Result<(), DaoError> {
    conn.execute(
        "DELETE FROM code_snippets WHERE id = ?",
        params![snippet_id],
    )?;
    Ok(())
}

//CRUD for dev logs
pub fn create_dev_log(conn: &Connection, dev_log: &DevLog) -> Result<i64, DaoError> {
    let language = "js";

    // Use the tag string directly from the dev_log struct
    let tags = match &dev_log.tags {
        None => String::new(),
        Some(tag) => tag.clone(),  // Just clone the tag without preprocessing
    };

    let processed_entry = preprocess_code(&dev_log.entry, language)
        .map_err(DaoError::PreprocessingError)?;

    conn.execute(
        "INSERT INTO dev_logs (entry, tags) VALUES (?, ?)",
        params![processed_entry, tags],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn read_dev_log(conn: &Connection, dev_log_id: i64) -> Result<Option<DevLog>, DaoError> {
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
    .optional().map_err(DaoError::from)
}

#[cfg(test)]
mod tests {
    use super::*; // This imports everything from the parent module (dao.rs)

    #[test]
    fn test_preprocess_code() {
        let raw_code = r#"console.log("Hello, it's me!");"#;
        let processed_code = preprocess_code(raw_code, "js").unwrap();
        assert_eq!(
            processed_code,
            "'''\nconsole.log(\"Hello, it''s me!\");\n'''"
        );
    }

    #[test]
    fn test_rust_preprocess_code() {
        let raw_code = r#"fn main() {
    // Path to the SQLite database file
    let db_path = "../clidblocal.db";

    // Create a new database connection
    let database = db::Database::new(db_path).expect("Failed to connect to the database ");

    //Initialize the database ( create tables if they don't exist)
    database.initialize().expect("Failed to initialize the database");
}"#;

        let processed_code = preprocess_code(raw_code, "rust").unwrap();

        // Check that the code was properly formatted with triple quotes
        assert!(processed_code.starts_with("'''\n"));
        assert!(processed_code.ends_with("\n'''"));

        // Check that single quotes were escaped correctly
        assert!(processed_code.contains("\"Failed to connect to the database \""));
        assert!(processed_code.contains("\"Failed to initialize the database\""));

        // Check that newlines are preserved
        assert!(processed_code.contains("Path to the SQLite database file"));
        assert!(processed_code.contains("Create a new database connection"));
    }
}
