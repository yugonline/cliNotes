use crate::models::{CodeSnippet, DevLog};
use rusqlite::{params, Connection, OptionalExtension};

fn preprocess_code(code: &str, language: &str) -> String {
    match language {
        "rust" | "ts" | "js" => {
            let escaped_code = code.replace("'", "''");
            format!("'''\n{}\n'''", escaped_code)
        }
        _ => panic!("Unsupported language."),
    }
}
//Languages only READ operation available

pub fn get_language_from_id(conn: &Connection, language_id: i64) -> Result<String, rusqlite::Error> {
    conn.query_row(
        "SELECT name FROM languages WHERE id = ?",
        params![language_id],
        |row| Ok(row.get::<_, String>(0)?),
    )
}

pub fn get_language_id_from_name(conn: &Connection, lang_name: &str) -> Result<i64, rusqlite::Error> {
    conn.query_row(
        "SELECT id FROM languages WHERE name = ?",
        params![lang_name],
        |row| Ok(row.get::<_, i64>(0)?),
    )
}

pub fn language_exists(conn: &Connection, language_name: &str) -> Result<bool, rusqlite::Error> {
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
) -> Result<i64, rusqlite::Error> {
    let lang_id = get_language_id_from_name(conn, lang_name)?;

    conn.execute(
        "INSERT INTO code_snippets (full_code, language_id) VALUES (?, ?)",
        params![preprocess_code(&snippet.full_code, lang_name), lang_id],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn read_code_snippet(
    conn: &Connection,
    snippet_id: i64,
) -> Result<Option<CodeSnippet>, rusqlite::Error> {
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
    ).optional()
}

pub fn update_code_snippet(
    conn: &Connection,
    snippet: &CodeSnippet,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE code_snippets SET full_code = ?, language_id = ? WHERE id = ?",
        params![&snippet.full_code, &snippet.language_id, &snippet.id],
    )?;
    Ok(())
}

pub fn delete_code_snippet(conn: &Connection, snippet_id: i64) -> Result<(), rusqlite::Error> {
    conn.execute(
        "DELETE FROM code_snippets WHERE id = ?",
        params![snippet_id],
    )?;
    Ok(())
}

//CRUD for dev logs
pub fn create_dev_log(conn: &Connection, dev_log: &DevLog) -> Result<i64, rusqlite::Error> {
    let language = "js";

    let tags = match &dev_log.tags {
        None => String::new(),
        Some(tag) => preprocess_code(tag, language),
    };

    conn.execute(
        "INSERT INTO dev_logs (entry, tags) VALUES (?, ?)",
        params![preprocess_code(&dev_log.entry, language), tags],
    )?;
    Ok(conn.last_insert_rowid())
}

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
    use super::*; // This imports everything from the parent module (dao.rs)

    #[test]
    fn test_preprocess_code() {
        let raw_code = r#"console.log("Hello, it's me!");"#;
        let processed_code = preprocess_code(raw_code, "js");
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
        let expected_code = "'''\nfn main() {\n    // Path to the SQLite database file\n    let db_path = \"../clidblocal.db\";\n\n    // Create a new database connection\n    let database = db::Database::new(db_path).expect(\"Failed to connect to the database \");\n\n    //Initialize the database ( create tables if they don''t exist)\n    database.initialize().expect(\"Failed to initialize the database\");\n}\n'''";

        let processed_code = preprocess_code(raw_code, "rust");
        assert_eq!(processed_code, expected_code);
    }
}
