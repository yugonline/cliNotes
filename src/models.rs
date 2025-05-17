use chrono::{NaiveDateTime};
use clap::Parser;
use diesel::prelude::*;

use crate::schema::*;

/// Arguments for creating a new dev log via the CLI
#[derive(Parser, Debug)]
pub struct DevLogArgs {
    /// The content of the dev log entry
    #[clap(long)]
    pub entry: String,

    /// Relevant tags for your logs
    #[clap(long, required = false)]
    pub tags: Option<String>,
}

/// Model representing a dev log stored in the database
#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = dev_logs)]
pub struct DevLog {
    pub id: i32,
    pub entry: String,
    pub date: NaiveDateTime,
    pub tags: Option<String>,
}

/// Struct used for inserting a new dev log
#[derive(Insertable, Debug)]
#[diesel(table_name = dev_logs)]
pub struct NewDevLog<'a> {
    pub entry: &'a str,
    pub tags: Option<&'a str>,
}

impl<'a> From<&'a DevLogArgs> for NewDevLog<'a> {
    fn from(args: &'a DevLogArgs) -> Self {
        Self {
            entry: &args.entry,
            tags: args.tags.as_deref(),
        }
    }
}
