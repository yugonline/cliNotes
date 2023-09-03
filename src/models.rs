use chrono::prelude::*;

pub struct DevLog {
    pub id: i64,
    pub entry: String,
    pub date: DateTime<Local>,
    pub tags: Option<String>,
}

impl DevLog {
    pub fn new(entry: String, tags: Option<String>) -> Self {
        DevLog {
            id: -1,
            entry,
            date: Local::now(),
            tags,
        }
    }
}

pub struct Language {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub struct CodeSnippet {
    pub id: i64,
    pub full_code: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub language_id: i64,
}

pub struct LearningNote {
    pub id: i64,
    pub file_path: String,
    pub file_name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub struct SnippetUsed {
    pub id: i64,
    pub snippet_id: i64,
    pub description: String,
    pub learning_note_id: Option<i64>,
    pub devlog_id: Option<i64>,
}
