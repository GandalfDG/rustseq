use std::{collections::HashMap, ops::Deref};
use std::rc::Rc;
use std::cell::RefCell;

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
        let mut subtrees: Vec<Rc<RefCell<Tree<i64>>>> = Vec::new();

        // each leaf node becomes the tip of its own subtree
        for block_id in leaf_block_ids {
            subtrees.push(
                Rc::new(
                    RefCell::new(
                        Tree {
                            value: block_id,
                            children: Vec::new()
                        }
                    )
                )
            );
        }

        let mut visited_nodes: Vec<Rc<RefCell<Tree<i64>>>> = Vec::new();
        for blockref in subtrees.iter() {
            visited_nodes.push(Rc::clone(blockref));
        } 

        // TODO store smart pointers to each tree node so we can access any processed 
        // node from a subtree to get around the node duplication behavior
        // subtrees from subtrees will be joined into new subtrees here
        // attach subtrees to their parents up until they reach the root
        let mut new_subtrees: Vec<Rc<RefCell<Tree<i64>>>> = Vec::new();

        while visited_nodes.len() <= (self.block_data.len())  {

            for subtree_ref in subtrees.iter() {
                // find the parent ID for the root of the subtree
                let block = self.block_data.get(&subtree_ref.borrow().value).unwrap();
                let parent_id = block.parent_id.unwrap_or(0);

                // is the parent node a node we've already visited?
                let visited_node = visited_nodes.iter().find(|tree| {
                    tree.borrow().value == parent_id
                });

                match visited_node {
                    // if we've visited this node, attach the current subtree as a child of the node
                    Some(node) => {
                        let subtree = subtree_ref.borrow().clone();
                        node.borrow_mut().children.push(subtree);
                    }
                    // otherwise, create a node for the parent
                    None => {
                        let new_subtree = Tree{
                            value: parent_id,
                            children: vec![subtree_ref.borrow().clone()]
                        };
                        let new_subtree_ref = Rc::new(
                            RefCell::new(new_subtree)
                        );
                        new_subtrees.push(Rc::clone(&new_subtree_ref));
                        visited_nodes.push(Rc::clone(&new_subtree_ref));
                    }
                }

                // add our node to visited
            }

            subtrees = new_subtrees.clone();
            new_subtrees.clear();

        }

        let final_tree = subtrees[0].clone();
        self.block_tree = final_tree.take();
        // println!("{:?}", nodes)
    }
}