pub struct DBInterface {
    connection: rusqlite::Connection
}

impl DBInterface {
    pub fn construct(file_name: &str) -> Result<Self, rusqlite::Error> {
        return Ok(DBInterface { connection: rusqlite::Connection::open(file_name)? });
    }

    pub fn init_db(&self) -> Result<(), rusqlite::Error> {
        self.connection.execute("CREATE TABLE IF NOT EXISTS blocks (hash TEXT, length INTEGER, last_hash TEXT, next_strength INTEGER, nonce TEXT, timestamp INTEGER)", [])?;
        Ok(())
    }

    pub fn execute<P: rusqlite::Params>(&self, query: &str, params: P) -> Result<(), rusqlite::Error> {
        self.connection.execute(query, params)?;
        Ok(())
    }

    pub fn execute_query<P, F, T>(&self, query: &str, params: P, func: F) -> Result<T, rusqlite::Error>
    where
        P: rusqlite::Params,
        F: FnOnce(&rusqlite::Row<'_>) -> Result<T, rusqlite::Error>,
    {
        return self.connection.query_row(query, params, func);
    }
}
