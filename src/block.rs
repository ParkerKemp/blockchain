use crate::db_block::DBBlock;
use crate::db_interface::DBInterface;

pub struct Block {
    pub hash: String,
    pub last_hash: String,
    pub next_strength: i32,
    pub nonce: String,
    pub timestamp: i32,

    db_block: DBBlock
}

impl Block {
    pub fn new(hash: String, last_hash: String, next_strength: i32, nonce: String, timestamp: i32, db_interface: DBInterface) -> Self {
        return Block {
            hash: hash,
            last_hash: last_hash,
            next_strength: next_strength,
            nonce: nonce,
            timestamp: timestamp,
            db_block: DBBlock { interface: db_interface }
        };
    }

    pub fn load(hash: String, db_interface: DBInterface) -> Result<Self, rusqlite::Error> {
        let db_block = DBBlock { interface: db_interface };

        let vals = db_block.load_block(&hash)?;

        return Ok(Block {
            hash: vals.0,
            last_hash: vals.1,
            next_strength: vals.2,
            nonce: vals.3,
            timestamp: vals.4,
            db_block: db_block
        });
    }

    pub fn save(&self) -> Result<(), rusqlite::Error> {
        self.db_block.insert_block(&self.hash, &self.last_hash, &self.next_strength, &self.nonce, &self.timestamp)?;

        Ok(())
    }

    pub fn print(&self) -> () {
        println!("hash: {}", &self.hash);
        println!("last_hash: {}", &self.last_hash);
        println!("next_strength: {}", &self.next_strength);
        println!("nonce: {}", &self.nonce);
        println!("timestamp: {}", &self.timestamp);
    }
}
