use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::Rc;
use std::option::Option;

use crate::db_interface::DBInterface;
use crate::block::Block;

pub struct BlockChain {
    interface: Rc<DBInterface>,
    newest: Option<Block>
}

impl BlockChain {
    pub fn new(interface: &Rc<DBInterface>) -> Self {
        return BlockChain {
            interface: Rc::clone(interface),
            newest: Option::None
        };
    }

    pub fn load_chain(&mut self) -> Result<bool, rusqlite::Error> {
        self.newest = Block::load_newest(&self.interface)?;

        if self.newest.is_none() {
            self.newest = Some(self.create_genesis());
            Ok(true)
        }
        else {
            Ok(self.verify_block(self.newest.as_ref().unwrap())?)
        }
    }

    pub fn verify_block(&self, block: &Block) -> Result<bool, rusqlite::Error> {
        if block.calc_hash() != block.hash.as_ref().unwrap() {
            return Ok(false);
        }

        return self.verify_block(&Block::load(&block.last_hash, &Rc::clone(&self.interface))?);
    }

    pub fn guess_next_block() -> () {
        
    }

    fn create_genesis(&self) -> Block {
        return Block::new(String::from("00000000000000000000000000000000"), 1, &Rc::clone(&self.interface));
    }

    fn current_unix_time() -> u64 {
        return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }

    fn nonce() -> String {
        let bytes = rand::thread_rng().gen::<[u8; 32]>();
        return String::from(std::str::from_utf8(&bytes).unwrap());
    }
}
