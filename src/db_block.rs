use crate::db_interface;

pub struct DBBlock {
    pub interface: db_interface::DBInterface
}

impl DBBlock {
    pub fn insert_block(&self, hash: &String, last_hash: &String, next_strength: &i32, nonce: &String, timestamp: &i32) -> Result<(), rusqlite::Error> {
        let query = "INSERT INTO blocks (hash, last_hash, next_strength, nonce, timestamp) VALUES (?, ?, ?, ?, ?)";
        self.interface.execute(query, rusqlite::params![hash, last_hash, next_strength, nonce, timestamp])?;
        Ok(())
    }
}

