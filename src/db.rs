use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::connection::SimpleConnection;

pub struct Database {
    conn: SqliteConnection,
}

impl Database {
    /// Creates a new database or opens an existing one
    pub fn new(db_path: &str) -> Result<Self, ConnectionError> {
        let conn = SqliteConnection::establish(db_path)?;
        Ok(Database { conn })
    }

    /// Initialize database by executing the SQL script if required
    pub fn initialize(&mut self) -> QueryResult<()> {
        let sql = include_str!("../sql/init.sql");
        self.conn.batch_execute(sql)
    }

    pub fn get_connection(&mut self) -> &mut SqliteConnection {
        &mut self.conn
    }
}
