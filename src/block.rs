use sha1::{Sha1, Digest};
use std::rc::Rc;

use crate::db_block::DBBlock;
use crate::db_interface::DBInterface;

pub struct Block {
    pub hash: Option<String>,
    pub length: i32,
    pub last_hash: String,
    pub next_strength: u8,
    pub nonce: String,
    pub timestamp: u64,

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

    fn load_from_row(row: (String, i32, String, u8, String, u64), db_block: DBBlock) -> Self {
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

    pub fn roll(&mut self, next_strength: u8, nonce: String, timestamp: u64) -> &String {
        self.next_strength = next_strength;
        self.nonce = nonce;
        self.timestamp = timestamp;
        self.hash = Some(self.calc_hash());
        return self.hash.as_ref().unwrap();
    }

    pub fn calc_hash(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.update(self.serialize().unwrap().as_slice());
        return hex::encode(hasher.finalize());
    }

    fn serialize(&self) -> Result<Vec<u8>, hex::FromHexError> {
        // length + last_hash + next strength + nonce + timestamp
        Ok([
            self.length.to_be_bytes().to_vec(),
            hex::decode(&self.last_hash)?,
            self.next_strength.to_be_bytes().to_vec(),
            hex::decode(&self.nonce)?,
            self.timestamp.to_be_bytes().to_vec()
        ].concat())
    }

    pub fn save(&self) -> Result<(), rusqlite::Error> {
        self.db_block.insert_block(&self.calc_hash(), &self.length, &self.last_hash, &self.next_strength, &self.nonce, &self.timestamp)?;

        Ok(())
    }

    pub fn print(&self, duration_secs: f64, count: &u64, strength: &u8) -> () {
        println!();
        println!("===================================");
        println!("hash: {}", &self.calc_hash());
        //println!("length: {}", &self.length);
        //println!("last_hash: {}", &self.last_hash);
        //println!("nonce: {}", &self.nonce);
        //println!("timestamp: {}", &self.timestamp);
        println!("serialized block: {}", hex::encode(&self.serialize().unwrap().as_slice()));
        println!("strength: {}", strength);
        println!("next strength: {}", &self.next_strength);
        println!();
        println!("{} guesses in {}s: {}", count, duration_secs, Self::format_hashrate(&duration_secs, &count));
        println!("===================================");
    }

    fn format_hashrate(duration_secs: &f64, count: &u64) -> String {
        let mut rate = *count as f64 / duration_secs;

        if rate < 1000f64 {
            return format!("{:.3} H/s", rate);
        }

        rate /= 1000f64;

        if rate < 1000f64 {
            return format!("{:.3} kH/s", rate);
        }

        rate /= 1000f64;

        if rate < 1000f64 {
            return format!("{:.3} MH/s", rate);
        }

        rate /= 1000f64;

        if rate < 1000f64 {
            return format!("{:.3} GH/s", rate);
        }

        rate /= 1000f64;

        if rate < 1000f64 {
            return format!("{:.3} TH/s", rate);
        }

        rate /= 1000f64;

        return format!("{:.3} PH/s", rate);
    }
}
