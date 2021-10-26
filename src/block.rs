use std::rc::Rc;

use crate::db_block::DBBlock;
use crate::db_interface::DBInterface;

pub struct Block {
    pub hash: Option<String>,
    pub length: i32,
    pub last_hash: String,
    pub next_strength: i32,
    pub nonce: String,
    pub timestamp: i32,

    db_block: DBBlock
}

impl Block {
    pub fn new(last_hash: String, length: i32, db_interface: &Rc<DBInterface>) -> Self {
        return Block {
            hash: None,
            length: length,
            last_hash: last_hash,
            next_strength: 0,
            nonce: String::from(""),
            timestamp: 0,
            db_block: DBBlock { interface: Rc::clone(db_interface) }
        };
    }

    pub fn load(hash: &String, db_interface: &Rc<DBInterface>) -> Result<Self, rusqlite::Error> {
        let db_block = DBBlock { interface: Rc::clone(db_interface) };

        let vals = db_block.load_block(&hash)?;

        return Ok(Block::load_from_row(vals, db_block));
    }

    pub fn load_newest(db_interface: &Rc<DBInterface>) -> Result<Option<Self>, rusqlite::Error> {
        let db_block = DBBlock { interface: Rc::clone(db_interface) };

        match db_block.load_newest() {
            Ok(vals) => {
                Ok(Some(Block::load_from_row(vals, db_block)))
            },
            Err(e) => {
                match e {
                    rusqlite::Error::QueryReturnedNoRows => {
                        Ok(None)
                    },
                    _ => {
                        Err(e)
                    }
                }
            }
        }

    }

    fn load_from_row(row: (String, i32, String, i32, String, i32), db_block: DBBlock) -> Self {
        return Block {
            hash: Some(row.0),
            length: row.1,
            last_hash: row.2,
            next_strength: row.3,
            nonce: row.4,
            timestamp: row.5,
            db_block: db_block
        };
    }

    pub fn calc_hash(&self) -> &String {
        return self.hash.as_ref().unwrap();
    }

    pub fn save(&self) -> Result<(), rusqlite::Error> {
        self.db_block.insert_block(&self.calc_hash(), &self.length, &self.last_hash, &self.next_strength, &self.nonce, &self.timestamp)?;

        Ok(())
    }

    pub fn print(&self) -> () {
        println!("hash: {}", &self.calc_hash());
        println!("length: {}", &self.length);
        println!("last_hash: {}", &self.last_hash);
        println!("next_strength: {}", &self.next_strength);
        println!("nonce: {}", &self.nonce);
        println!("timestamp: {}", &self.timestamp);
    }
}
