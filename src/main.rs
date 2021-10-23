mod db_interface;

fn main() -> Result<(), rusqlite::Error> {
	let db_interface = db_interface::DBInterface::construct("chain.db")?;
    db_interface.init_db()?;
    Ok(())
}
