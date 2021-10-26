mod db_block;
mod db_interface;
mod block;
mod block_chain;

use std::rc::Rc;

fn main() -> Result<(), rusqlite::Error> {
	let db_interface = Rc::new(db_interface::DBInterface::construct("chain.db")?);
    db_interface.init_db()?;

    let mut block_chain = block_chain::BlockChain::new(&Rc::clone(&db_interface));
    if block_chain.load_chain()? {
        println!("Successfully loaded chain");
    } else {
        println!("Failed to load chain");
    }
    //let block = block::Block::load(String::from("asdf"), &Rc::clone(&db_interface))?;
    //block.print();
    //let block = block::Block::new(String::from("asdf"), String::from("1234"), 10, String::from("2345"), 12345, db_interface);
    //block.save()?;
    Ok(())
}
