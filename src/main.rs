mod db_block;
mod db_interface;
mod block;

fn main() -> Result<(), rusqlite::Error> {
	let db_interface = db_interface::DBInterface::construct("chain.db")?;
    db_interface.init_db()?;

    let block = block::Block::load(String::from("asdf"), db_interface)?;
    block.print();
    //let block = block::Block::new(String::from("asdf"), String::from("1234"), 10, String::from("2345"), 12345, db_interface);
    //block.save()?;
    Ok(())
}
