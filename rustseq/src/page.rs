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

    fn get_leaves(&self, blocks_by_parent: &HashMap<i64, Vec<i64>>) -> Vec<i64> {
        let mut leaf_block_ids = Vec::new();
        
        for (block_id, _block) in self.block_data.iter() {
            // check that no block has this block as a parent
            if let None = blocks_by_parent.get(block_id) {
                leaf_block_ids.push(*block_id)
            }
        }

        return leaf_block_ids;
    }

    /// create a tree of Tree<usize> nodes representing the blocks of the page.
    /// Each node contains the ID of a block and a vector of child Tree<usize>
    /// nodes.
    pub fn build_tree(&mut self) {
        // hash map containing child block ID vectors for each parent ID
        let mut blocks_by_parent: HashMap<i64, Vec<i64>> = HashMap::new();

        for (block_id, block) in self.block_data.iter() {
            let parent_id = block.parent_id.unwrap_or(0);
            if let None = blocks_by_parent.get_mut(&parent_id) {
                blocks_by_parent.insert(parent_id, Vec::new());
            }
            blocks_by_parent.get_mut(&parent_id).unwrap().push(*block_id);
        }

        let leaf_block_ids = self.get_leaves(&blocks_by_parent);

        // key is tree parent ID, value is the subtree itself
        let mut subtrees: HashMap<i64, Tree<i64>> = HashMap::new();

        // attach the leaves to their parents
        for block_id in leaf_block_ids {

            // the data associated with the leaf block id
            let block = self.block_data.get(&block_id).unwrap();

            // the block ID of the parent of the current block
            let parent_id = block.parent_id.unwrap_or(0);
            
            if let None = subtrees.get(&parent_id) {
                subtrees.insert(parent_id, Tree { value: parent_id, children: Vec::new() });
            }

            let parent_tree = subtrees.get_mut(&parent_id).unwrap();
            parent_tree.children.push(Tree { value: block_id, children: Vec::new() });
        }

        // subtrees from subtrees will be joined into new subtrees here
        // attach subtrees to their parents up until they reach the root
        while subtrees.len() > 1 {
            let mut next_subtrees: HashMap<i64, Tree<i64>> = HashMap::new();
            for subtree_root_id in subtrees.keys() {

                let subtree = subtrees.get(subtree_root_id).unwrap().clone();

                // subtree with value 0 is already at the root
                if subtree.value == 0 {
                    next_subtrees.insert(0, subtree);
                } 

                // non-root subtree
                else {
                    let optblock = self.block_data.get(&subtree.value);
                    let parent_id;
                    match optblock {
                        Some(block) => {
                            parent_id = block.parent_id.unwrap_or(0);
                        }
                        None => {
                            parent_id = 0;
                        }
                    }

                    match next_subtrees.get_mut(&parent_id) {
                        Some(parent_subtree) => {
                            parent_subtree.children.push(subtree);
                        }
                        None => {
                            next_subtrees.insert(parent_id, Tree {
                                value: parent_id,
                                children: vec![subtree]
                            });
                        }
                    }
                }
            }

            subtrees = next_subtrees;
        }

        self.block_tree = subtrees.get(&0).unwrap().clone();



        let tree_vec: Vec<&i64> = self.block_tree.dfs_preorder_iter().collect();
        println!("{:?}", tree_vec);
    }
}