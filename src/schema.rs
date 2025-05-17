// @generated automatically by Diesel CLI.
// This file defines database schema for Diesel ORM.

use diesel::table;

// Table definitions

table! {
    dev_logs (id) {
        id -> Integer,
        entry -> Text,
        date -> Timestamp,
        tags -> Nullable<Text>,
    }
}

table! {
    languages (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    code_snippets (id) {
        id -> Integer,
        full_code -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        language_id -> Integer,
    }
}

table! {
    learning_notes (id) {
        id -> Integer,
        file_path -> Text,
        file_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    snippets_used (id) {
        id -> Integer,
        snippet_id -> Integer,
        description -> Text,
        learning_note_id -> Nullable<Integer>,
        devlog_id -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    dev_logs,
    languages,
    code_snippets,
    learning_notes,
    snippets_used,
);
