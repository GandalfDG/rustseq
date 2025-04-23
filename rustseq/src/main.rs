mod block;
mod db;

use std::fs;

use block::Block;
use db::DB;


const TEST_DB_PATH: &str = "test_db.sqlite";

fn init_test_db() -> DB {
    // delete the existing test db
    fs::remove_file(TEST_DB_PATH).unwrap_or_default();

    // connect to the DB which creates the file
    let database = DB::connect(TEST_DB_PATH);

    // run SQL to create the tables defined for rustseq
    database.create_tables();

    return database;
}

fn main() {
    let mut database = init_test_db();

    let block1 = Block::new(1, "Hello, world!");
    let mut block2 = Block::new(2, "This is a child block");
    let mut block3 = Block::new(3, "this is a sibling");

    block2.set_parent(&block1);
    block3.set_parent(&block1);
    block2.set_sibling(&block3);

    println!("{:?}", block2);
    println!("{:?}", block1.as_block_row());
    println!("{:?}", block2.as_block_row());
    println!("{:?}", block3.as_block_row());


    let mut block_id = database.insert_block(&mut block1.as_block_row()).unwrap();
    println!("{:?}", block_id);
    block_id = database.insert_block(&mut block2.as_block_row()).unwrap();
    println!("{:?}", block_id);
    block_id = database.insert_block(&mut block3.as_block_row()).unwrap();
    println!("{:?}", block_id);
}
