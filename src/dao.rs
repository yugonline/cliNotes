use crate::models::{CodeSnippet, JournalEntry};
use rusqlite::{params, Connection, OptionalExtension};
use std::fmt;

// AI placeholder function - simulates AI processing
pub fn call_journal_ai(entry_text: &str) -> (String, String) {
    // This is a placeholder implementation
    // In a real implementation, this would call an external AI service
    
    // Simple sentiment analysis based on keywords
    let sentiment = if entry_text.to_lowercase().contains("happy") || 
                      entry_text.to_lowercase().contains("good") || 
                      entry_text.to_lowercase().contains("great") ||
                      entry_text.to_lowercase().contains("excited") {
        "positive".to_string()
    } else if entry_text.to_lowercase().contains("sad") || 
              entry_text.to_lowercase().contains("bad") || 
              entry_text.to_lowercase().contains("terrible") ||
              entry_text.to_lowercase().contains("frustrated") {
        "negative".to_string()
    } else {
        "neutral".to_string()
    };
    
    // Simple tag generation based on common programming keywords
    let mut ai_tags = Vec::new();
    if entry_text.to_lowercase().contains("rust") { ai_tags.push("rust"); }
    if entry_text.to_lowercase().contains("project") { ai_tags.push("project"); }
    if entry_text.to_lowercase().contains("bug") { ai_tags.push("debugging"); }
    if entry_text.to_lowercase().contains("learn") { ai_tags.push("learning"); }
    if entry_text.to_lowercase().contains("code") { ai_tags.push("coding"); }
    if entry_text.to_lowercase().contains("work") { ai_tags.push("work"); }
    
    let ai_tags_string = if ai_tags.is_empty() {
        "general".to_string()
    } else {
        ai_tags.join(",")
    };
    
    (sentiment, ai_tags_string)
}

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
    lang_name: &str,
) -> Result<(), DaoError> {
    let processed_code = preprocess_code(&snippet.full_code, lang_name)
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

//CRUD for journal entries
pub fn create_journal_entry(conn: &Connection, journal_entry: &JournalEntry) -> Result<i64, DaoError> {
    // Use the tag string directly from the journal_entry struct
    let tags = match &journal_entry.tags {
        None => String::new(),
        Some(tag) => tag.clone(),  // Just clone the tag without preprocessing
    };

    // Call AI function to get sentiment and AI tags
    let (sentiment, ai_tags) = call_journal_ai(&journal_entry.entry);

    conn.execute(
        "INSERT INTO journal_entries (entry, tags, sentiment, ai_tags) VALUES (?, ?, ?, ?)",
        params![&journal_entry.entry, tags, sentiment, ai_tags],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn read_journal_entry(conn: &Connection, journal_entry_id: i64) -> Result<Option<JournalEntry>, DaoError> {
    conn.query_row(
        "SELECT id, entry, date, tags, sentiment, ai_tags FROM journal_entries WHERE id = ?1",
        params![journal_entry_id],
        |row| {
            Ok(JournalEntry {
                id: row.get(0)?,
                entry: row.get(1)?,
                date: row.get(2)?,
                tags: row.get(3)?,
                sentiment: row.get(4)?,
                ai_tags: row.get(5)?,
            })
        },
    )
    .optional().map_err(DaoError::from)
}

pub fn get_journal_entries_by_period(conn: &Connection, period: &str) -> Result<Vec<JournalEntry>, DaoError> {
    let query = match period {
        "week" => "SELECT id, entry, date, tags, sentiment, ai_tags FROM journal_entries WHERE date >= date('now', '-7 days') ORDER BY date DESC",
        "month" => "SELECT id, entry, date, tags, sentiment, ai_tags FROM journal_entries WHERE date >= date('now', '-1 month') ORDER BY date DESC",
        "year" => "SELECT id, entry, date, tags, sentiment, ai_tags FROM journal_entries WHERE date >= date('now', '-1 year') ORDER BY date DESC",
        _ => return Err(DaoError::PreprocessingError(format!("Invalid period: {}", period))),
    };

    let mut stmt = conn.prepare(query)?;
    let journal_iter = stmt.query_map([], |row| {
        Ok(JournalEntry {
            id: row.get(0)?,
            entry: row.get(1)?,
            date: row.get(2)?,
            tags: row.get(3)?,
            sentiment: row.get(4)?,
            ai_tags: row.get(5)?,
        })
    })?;

    let mut entries = Vec::new();
    for entry in journal_iter {
        entries.push(entry?);
    }
    Ok(entries)
}

pub fn search_journal_entries(conn: &Connection, query: &str) -> Result<Vec<JournalEntry>, DaoError> {
    let search_query = format!("%{}%", query.to_lowercase());
    
    let mut stmt = conn.prepare(
        "SELECT id, entry, date, tags, sentiment, ai_tags FROM journal_entries 
         WHERE LOWER(entry) LIKE ?1 OR LOWER(tags) LIKE ?1 OR LOWER(ai_tags) LIKE ?1 
         ORDER BY date DESC"
    )?;
    
    let journal_iter = stmt.query_map([search_query], |row| {
        Ok(JournalEntry {
            id: row.get(0)?,
            entry: row.get(1)?,
            date: row.get(2)?,
            tags: row.get(3)?,
            sentiment: row.get(4)?,
            ai_tags: row.get(5)?,
        })
    })?;

    let mut entries = Vec::new();
    for entry in journal_iter {
        entries.push(entry?);
    }
    Ok(entries)
}



use std::collections::HashSet;
use crate::models::JournalSummary;

pub fn summarize_journal_entries(entries: &Vec<JournalEntry>) -> JournalSummary {
    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut neutral_count = 0;
    let mut all_ai_tags = Vec::new();

    for entry in entries {
        match entry.sentiment.as_deref() {
            Some("positive") => positive_count += 1,
            Some("negative") => negative_count += 1,
            _ => neutral_count += 1,
        }

        if let Some(ai_tags) = &entry.ai_tags {
            all_ai_tags.extend(ai_tags.split(',').map(|s| s.trim()));
        }
    }

    let common_topics = all_ai_tags.into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .take(5)
        .collect::<Vec<_>>()
        .join(", ");

    JournalSummary {
        total_entries: entries.len(),
        positive_count,
        negative_count,
        neutral_count,
        common_topics,
    }
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
