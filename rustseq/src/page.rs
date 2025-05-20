
use std::{collections::HashMap};


use tree_iterators_rs::prelude::*;
use streaming_iterator::StreamingIteratorMut;

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

    /// iterate through the built tree and sort child nodes by their
    /// next_sibling fields
    /// return a properly sorted tree
    fn sort_children(&self, mut built_tree: Tree<i64>) -> Tree<i64> {
        let mut tree_iter = built_tree.dfs_preorder_iter_mut().attach_context();

        while let Some(node) = tree_iter.next_mut() {
            let mut children = node.children_mut();
            
        }

        Tree{value: 0, children:Vec::new()}
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
        let mut leaves: Vec<Tree<i64>> = Vec::new();

        // each leaf block becomes a Tree node owned by the leaves vector
        for block_id in leaf_block_ids {
            leaves.push(
                Tree {
                    value: block_id,
                    children: Vec::new()
                }
            );
        }

        // vector of complete subtrees
        let mut subtrees: Vec<Tree<i64>> = Vec::new();

        // store which subtree each block's node is in
        // key: block id, value: subtree vector index
        let mut known_nodes: HashMap<i64, usize> = HashMap::new();

        // OK, now try for each leaf getting to the root before going to another leaf
        // subtrees from `subtrees` will be joined into new subtrees here
        // attach subtrees to their parents up until they reach the root

        // Take the leaf from the vector
        for (subtree_index, leaf) in leaves.into_iter().enumerate() {
            // go from the leaf until reaching the root or until reaching an already visited node
            let mut current_node_opt: Option<Tree<i64>> = Some(leaf);

            while let Some(ref current_node) = current_node_opt {
                // build the subtree up until reaching the root or a known node

                // store the current node value in the known_nodes map
                known_nodes.insert(current_node.value, subtree_index);

                // get the block data of the current node
                let current_node_block_data = self.block_data.get(&current_node.value).expect("the current node must refer to a block in block_data");

                // the parent ID is a value, and if not it's a child of the root node
                let parent_id_opt = current_node_block_data.parent_id;

                match parent_id_opt {
                    Some(parent_id) => {
                        // we are not yet at the root node
                        // is the parent node a known node?
                        if let Some(known_node_subtree_index) = known_nodes.get(&parent_id) {
                            // iterate the indicated subtree to find the node, append the current node
                            // to its children and break
                            let known_node_subtree = subtrees.get_mut(*known_node_subtree_index).expect("if it's a known node, the subtree must exist in subtrees");
                            let mut known_node_iter = known_node_subtree.dfs_preorder_iter_mut().attach_context();

                            while let Some(context) = known_node_iter.next_mut() {

                                // the context ancestors field includes the current node's value
                                let current_node_value = context.ancestors().last().unwrap();
                                if **current_node_value !=  parent_id { continue; }

                                let children = context.children_mut();
                                children.push(current_node.clone());
                                current_node_opt = None;
                                break;
                            }

                        } else {
                            // create the parent node, update current_node and continue
                            let parent_node = Tree {
                                value: parent_id,
                                children: vec![current_node.clone()]
                            };

                            current_node_opt = Some(parent_node);
                        }
                    }
                    None => {
                        // the current node is a child of the root node
                        // store this subtree in subtrees
                        subtrees.push(current_node.clone());
                        break;
                    }
                    
                }
            }
        }
        self.block_tree = subtrees.remove(0);
    }

    pub fn print_tree(&self) {
        let nodes: Vec<&i64> = self.block_tree.dfs_preorder_iter().collect();
        println!("{nodes:?}");
    }
}