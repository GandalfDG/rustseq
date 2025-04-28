use tree_iterators_rs::prelude::*;

use crate::db::{BlockRow, PageRow};

/// A page is represented by a page object from the
/// database, and a tree of block objects with the data
/// for each node contained in the block_data vector
pub struct Page {
    page_data: PageRow,
    block_data: Vec<BlockRow>,
    block_tree: Tree<usize>
}

impl Page {
    pub fn new(page_row: PageRow, block_data: Vec<BlockRow>) -> Self {

        let root_block_index = block_data.iter()
        .position(|block| {
            block.id.is_some() && 
            block.id.unwrap() == page_row.root_block_id.unwrap()
        }).unwrap();
        
        Page {
            page_data: page_row,
            block_data: block_data,
            block_tree: Tree {
                value: root_block_index,
                children: Vec::new()
            }
        }
    
    }

}