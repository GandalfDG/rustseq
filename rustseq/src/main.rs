mod block;
mod db;

use block::Block;
use db::DB;

fn main() {
    println!("Hello, world!");

    let block1 = Block::new(1, "Hello, world!");
    let mut block2 = Block::new(2, "This is a child block");
    let mut block3 = Block::new(3, "this is a sibling");

    block2.set_parent(&block1);
    block3.set_parent(&block1);
    block2.set_sibling(&block3);

    let database = DB::connect("test_db.sqlite");
    database.create_tables();

    database.store_block(&block1);
    database.store_block(&block2);
    database.store_block(&block3);
    println!("{:?}", block2);
    println!("{:?}", block1.as_block_row());
    println!("{:?}", block2.as_block_row());
    println!("{:?}", block3.as_block_row());

}
