use std::collections::HashMap;

use tree_iterators_rs::prelude::*;

use crate::db::{BlockRow, PageRow};

/// A page is represented by a page object from the
/// database, and a tree of block objects with the data
/// for each node contained in the block_data vector
#[derive(Debug)]
pub struct Page {
    page_data: PageRow,
    block_data: HashMap<i64, BlockRow>,
    block_tree: Tree<usize>,
}

impl Page {
    pub fn new(page_row: PageRow, block_data: Vec<BlockRow>) -> Self {

        let mut block_map = HashMap::new();
        
        for block in block_data.into_iter() {
            block_map.insert(block.id.unwrap(), block);
        }

        block_map.insert(0, BlockRow{
            id: None, 
            content: String::from(""), 
            parent_id: None, 
            sibling_id: None, 
            page_id: page_row.id
        });
        
        Page {
            page_data: page_row,
            block_data: block_map,
            block_tree: Tree {
                value: 0,
                children: Vec::new()
            }
        }
    }

    pub fn set_root_block(&mut self, root_block_id: Option<i64>) {
        self.page_data.root_block_id = root_block_id;
    }

    /// from the root block build the tree based on parent and sibling
    /// ID fields of the blocks in block_data
    /// Since our DB representation holds parent and sibling, a top-down
    /// approach doesn't really make sense.
    /// 
    /// for each parent_id create a linked list of siblings and put those
    /// into a map keyed by parent_id
    /// 
    /// for each parent_id list create a Tree with the parent ID and the 
    /// children as Trees
    pub fn build_tree(&mut self) {
        let mut block_id_map: HashMap<i64, Tree<i64>> = HashMap::new();
        let mut parent_id_map: HashMap<i64, Vec<i64>> = HashMap::new();

        // create a map of Tree nodes by block ID for fast lookup
        // and a map of parent block IDs to lists of child IDs
        for (block_id, block) in self.block_data.iter() {
            let id = *block_id;
            block_id_map.insert(id, Tree {
                value: id,
                children: Vec::new()
            });

            if let Some(parent_id) = block.parent_id {
                parent_id_map.entry(parent_id).or_insert(Vec::new()).push(id);
            };
        }

        // create subtrees indexed by the id of their root node
        let mut subtree_map: HashMap<i64, Tree<i64>> = HashMap::new();

        // attach children to parents
        for(parent_id, child_ids) in parent_id_map {
            // get a parent node
            let mut parent_tree = block_id_map.get(&parent_id).unwrap().clone();
            for child_id in child_ids {
                parent_tree.children.push(block_id_map.get(&child_id).unwrap().clone())
            }
            
            subtree_map.insert(parent_id, parent_tree);
        }

        //


        // for (_id, subtree) in lookup.iter() {
        //     let block_data = self.block_data.get(subtree.value).unwrap();
        //     let parent = lookup.get(&block_data.parent_id.unwrap()).unwrap();
        //     parent.children.push(subtree.clone());
        // }

    }
}