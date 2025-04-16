mod block;

use block::Block;

fn main() {
    println!("Hello, world!");

    let block1 = Block::new("Hello, world!");
    let block2 = Block::new("This is a child block");

    block2.set_parent(&block1);

    println!("{:?}", block1);
}
