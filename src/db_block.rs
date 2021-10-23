use crate::db_interface::DBInterface;

pub struct DBBlock {
    pub interface: DBInterface,
}

impl DBBlock {
    pub fn insert_block(&self, hash: &String, last_hash: &String, next_strength: &i32, nonce: &String, timestamp: &i32) -> Result<(), rusqlite::Error> {
        let query = "INSERT INTO blocks (hash, last_hash, next_strength, nonce, timestamp) VALUES (?, ?, ?, ?, ?)";
        self.interface.execute(query, rusqlite::params![hash, last_hash, next_strength, nonce, timestamp])?;
        Ok(())
    }

    pub fn load_block(&self, hash: &String) -> Result<(String, String, i32, String, i32), rusqlite::Error> {
        let query = "SELECT hash, last_hash, next_strength, nonce, timestamp FROM blocks WHERE hash = ?";
        return self.interface.execute_query(
            query,
            rusqlite::params![hash],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                ))
            }
        );
    }
}

