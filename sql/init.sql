CREATE TABLE IF NOT EXISTS journal_entries (id INTEGER CONSTRAINT journal_entries_pk PRIMARY KEY AUTOINCREMENT, entry TEXT NOT NULL, date DATETIME DEFAULT CURRENT_TIMESTAMP, tags TEXT, sentiment TEXT, ai_tags TEXT);

CREATE TABLE IF NOT EXISTS languages (id INTEGER CONSTRAINT languages_pk PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL UNIQUE, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP);

CREATE TABLE IF NOT EXISTS code_snippets (id INTEGER PRIMARY KEY AUTOINCREMENT, full_code TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP, language_id INTEGER NOT NULL REFERENCES languages);

CREATE TRIGGER IF NOT EXISTS update_timestamp_after_update_code_snippets AFTER UPDATE ON code_snippets FOR EACH ROW BEGIN UPDATE code_snippets SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id; END;

CREATE TRIGGER IF NOT EXISTS update_timestamp_after_update_languages AFTER UPDATE ON languages FOR EACH ROW BEGIN UPDATE languages SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id; END;

CREATE TABLE IF NOT EXISTS learning_notes (id INTEGER CONSTRAINT learning_notes_pk PRIMARY KEY AUTOINCREMENT, file_path TEXT NOT NULL, file_name TEXT NOT NULL, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP);

CREATE TRIGGER IF NOT EXISTS update_timestamp_after_update_learning_notes AFTER UPDATE ON learning_notes FOR EACH ROW BEGIN UPDATE learning_notes SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id; END;

CREATE TABLE IF NOT EXISTS snippets_used (id INTEGER CONSTRAINT snippets_used_pk PRIMARY KEY AUTOINCREMENT, snippet_id INTEGER CONSTRAINT snippets_used_code_snippets_id_fk REFERENCES code_snippets ON DELETE CASCADE, description TEXT, learning_note_id INTEGER CONSTRAINT snippets_used_learning_notes_id_fk REFERENCES learning_notes ON DELETE CASCADE, journal_entry_id INTEGER CONSTRAINT snippets_used_journal_entries_id_fk REFERENCES journal_entries ON DELETE CASCADE);

INSERT OR IGNORE INTO languages (name) VALUES ('rust'), ('js'), ('ts'), ('python'), ('c++'), ('java'), ('go'), ('html'), ('css');
