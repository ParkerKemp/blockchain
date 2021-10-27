//use bitvec::{slice::BitSlice, order::Lsb0, bits};
use bit_array::BitArray;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::Rc;
use std::option::Option;
use std::mem::swap;

use crate::db_interface::DBInterface;
use crate::block::Block;

const GENESIS_HASH: &str = "00000000000000000000000000000000";

const TARGET_MINING_DURATION: u64 = 10;
const DURATION_MARGIN: u64 = 2;

pub struct BlockChain {
    interface: Rc<DBInterface>,
    last: Option<Block>,
    next: Option<Block>
}

impl BlockChain {
    pub fn new(interface: &Rc<DBInterface>) -> Self {
        return BlockChain {
            interface: Rc::clone(interface),
            last: Option::None,
            next: Option::None,
        };
    }

    pub fn load_chain(&mut self) -> Result<bool, rusqlite::Error> {
        self.last = Block::load_newest(&self.interface)?;

        if self.last.is_none() {
            self.next = Some(self.create_genesis());
            Ok(true)
        }
        else {
            self.next = Some(self.create_next());

            if self.last.as_ref().unwrap().last_hash != GENESIS_HASH {
                let parent_block = Block::load(&self.last.as_ref().unwrap().last_hash, &Rc::clone(&self.interface))?;

                Ok(self.verify_chain(&parent_block, self.last.as_ref())?)
            } else {
                Ok(true)
            }
        }
    }

    fn create_next(&self) -> Block {
        let last = self.last.as_ref().unwrap();
        return Block::new(last.hash.as_ref().unwrap().clone(), last.length + 1, &Rc::clone(&self.interface));
    }

    pub fn verify_chain(&self, block: &Block, child_block: Option<&Block>) -> Result<bool, rusqlite::Error> {
        // !!!!!!!!!!!! Still need to figure out how to verify hash strength. Since it is enforced "forward" and we are traversing "backward" we may need to track two blocks at once during this recursion

        if !Self::verify_block(Some(&block), child_block) {
            return Ok(false);
        }

        if block.last_hash == GENESIS_HASH {
            return Ok(true);
        }

        let parent_block = Block::load(&block.last_hash, &Rc::clone(&self.interface))?;

        return self.verify_chain(&parent_block, Some(block));
    }

    pub fn verify_block(parent_block: Option<&Block>, block: Option<&Block>) -> bool {
        let unwrapped_block = block.as_ref().unwrap();

        if &unwrapped_block.calc_hash() != unwrapped_block.hash.as_ref().unwrap() {
            return false;
        }

        return true;
    }

    fn check_strength(hash: &String, strength: &u8) -> bool {
        let mut i = 0u8;

        let hex = hex::decode(&hash).unwrap();
        let bits: BitArray<u32, typenum::U160>  = BitArray::from_bytes(&hex);

        while &i < strength {
            if bits.get(usize::from(i)).unwrap() == true {
                return false;
            }

            i += 1;
        }

        return true;
    }

    fn calc_next_strength(&self, current_time: u64) -> u8 {
        let target_strength = self.required_strength();

        match &self.last {
            Some(last) => {
                let duration = current_time - last.timestamp;

                if duration < TARGET_MINING_DURATION - DURATION_MARGIN {
                    return target_strength + 1;
                }

                if duration > TARGET_MINING_DURATION + DURATION_MARGIN && target_strength > 0 {
                    return target_strength - 1;
                }

                return target_strength;
            },
            None => 1
        }
    }

    fn required_strength(&self) -> u8 {
        match &self.last {
            Some(last) => last.next_strength,
            None => 1
        }
    }

    pub fn guess_next_block(&mut self) -> () {
        let current_time = Self::current_unix_time();
        let target_strength = self.required_strength();
        let next_strength = self.calc_next_strength(current_time);

        // Apparently this is how you should dereference an Option<T> https://stackoverflow.com/questions/27361350/calling-a-method-on-a-value-inside-a-mutable-option
        if let Some(next) = &mut self.next {
            next.roll(next_strength, Self::nonce(), current_time);

            if Self::check_strength(next.hash.as_ref().unwrap(), &target_strength) {
                next.print();
                let duration = current_time - match &self.last {
                    Some(last) => last.timestamp,
                    None => current_time
                };

                println!("Duration: {}", duration);
                next.save();
                swap(&mut self.last, &mut self.next);
                self.next = Some(self.create_next());
            }
        }
    }

    fn create_genesis(&self) -> Block {
        return Block::new(String::from(GENESIS_HASH), 1, &Rc::clone(&self.interface));
    }

    fn current_unix_time() -> u64 {
        return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    }

    fn nonce() -> String {
        let bytes = rand::thread_rng().gen::<[u8; 32]>();
        return hex::encode(&bytes);
    }
}
