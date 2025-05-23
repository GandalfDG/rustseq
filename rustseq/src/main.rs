mod page;
mod db;

use std::fs;

use db::{DB, PageRow, BlockRow};
use page::Page;

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

    // put a page in the database
    let mut page_row = PageRow{id: None, title: String::from("test page"), root_block_id: None};
    database.insert_page(&mut page_row).unwrap();

    // put some blocks in the database
    let mut block1 = BlockRow::new("Hello World", None, None, page_row.id);
    let mut block2 = BlockRow::new("This is a child block", None, None, page_row.id);
    let mut block3 = BlockRow::new("this is a sibling", None, None, page_row.id);
    let mut block4 = BlockRow::new("this is a sub-sibling", None, None, page_row.id);

    
    database.insert_block(&mut block1).unwrap();
    database.insert_block(&mut block2).unwrap();
    database.insert_block(&mut block3).unwrap();
    database.insert_block(&mut block4).unwrap();

    block2.parent_id = block1.id;
    block3.parent_id = block1.id;
    block4.parent_id = block3.id;

    //        1
    //      /  \
    //     2    3
    //         / 
    //        4

    database.update_block(&block2).unwrap();
    database.update_block(&block3).unwrap();
    database.update_block(&block4).unwrap();

    // get the page's blocks from the database
    let page_blocks = database.get_page_blocks(&page_row).unwrap();

    println!("{:#?}", page_blocks);

    // put those blocks into the Page tree structure
    let mut internal_page = Page::new(page_row, page_blocks);
    internal_page.build_tree();
    // internal_page.set_root_block(internal_page.get_block_data_ref()[0].id);
    internal_page.print_tree();

}
