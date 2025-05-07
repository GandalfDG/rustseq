use std::collections::HashMap;

use tree_iterators_rs::prelude::*;
use streaming_iterator::StreamingIterator;

use crate::db::{BlockRow, PageRow};

/// A page is represented by a page object from the
/// database, and a tree of block objects with the data
/// for each node contained in the block_data vector
#[derive(Debug)]
pub struct Page {
    page_data: PageRow,
    block_data: HashMap<i64, BlockRow>,
    block_tree: Tree<i64>,
}

impl Page {
    pub fn new(page_row: PageRow, block_data: Vec<BlockRow>) -> Self {

        let mut block_map = HashMap::new();
        
        for block in block_data.into_iter() {
            block_map.insert(block.id.unwrap(), block);
        }
        
        Page {
            page_data: page_row,
            block_data: block_map,
            block_tree: Tree {
                value: 0,
                children: Vec::new()
            }
        }
    }


    /// create a tree of Tree<usize> nodes representing the blocks of the page.
    /// Each node contains the ID of a block and a vector of child Tree<usize>
    /// nodes.
    pub fn build_tree(&mut self) {
        // create a mirror of the block_data map with block IDs to keep track of blocks
        // which haven't yet been added to the tree
        let mut remaining_blocks: Vec<i64> = self.block_data.keys().copied().collect();
        let mut blocks_by_parent: HashMap<i64, Vec<i64>> = HashMap::new();

        for (block_id, block) in self.block_data.iter() {
            let parent_id = block.parent_id.unwrap_or(0);
            if let None = blocks_by_parent.get_mut(&parent_id) {
                blocks_by_parent.insert(parent_id, Vec::new());
            }
            blocks_by_parent.get_mut(&parent_id).unwrap().push(*block_id);
        }

        let mut tree: Tree<i64> = Tree {
            value: 0,
            children: blocks_by_parent.remove(&0).unwrap().iter().map(|block_id| {
                Tree {
                    value: *block_id,
                    children: Vec::new()
                }
            }).collect()
        };

        let mut subtrees: HashMap<i64, Tree<i64>> = HashMap::new();
        subtrees.insert(0, tree);

        for (parent_id, child_id_vec) in blocks_by_parent.iter_mut() {
            subtrees.insert(*parent_id, child_id_vec.iter().map(|child| {
                Tree {
                    value: *child,
                    children: Vec::new()
                }
            }).collect());
        }

    }
}