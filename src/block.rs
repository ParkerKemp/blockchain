use crate::db_block;
use crate::db_interface;

pub struct Block {
    hash: String,
    last_hash: String,
    next_strength: i32,
    nonce: String,
    timestamp: i32,

    db_block: db_block::DBBlock
}

impl Block {
    pub fn new(hash: String, last_hash: String, next_strength: i32, nonce: String, timestamp: i32, db_interface: db_interface::DBInterface) -> Self {
        return Block {
            hash: hash,
            last_hash: last_hash,
            next_strength: next_strength,
            nonce: nonce,
            timestamp: timestamp,
            db_block: db_block::DBBlock { interface: db_interface }
        };
    }

    pub fn save(&self) -> Result<(), rusqlite::Error> {
        self.db_block.insert_block(&self.hash, &self.last_hash, &self.next_strength, &self.nonce, &self.timestamp)?;

        Ok(())
    }
}
