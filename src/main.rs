extern crate typenum;
mod db_block;
mod db_interface;
mod block;
mod block_chain;

use std::rc::Rc;

fn main() -> Result<(), rusqlite::Error> {
	let db_interface = Rc::new(db_interface::DBInterface::construct("chain.db")?);
    db_interface.init_db()?;

    let mut block_chain = block_chain::BlockChain::new(&Rc::clone(&db_interface));
    if !block_chain.load_chain()? {
        println!("Failed to load chain");
    }

    loop {
        block_chain.guess_next_block();
    }

    Ok(())
}
