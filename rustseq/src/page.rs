use tree_iterators_rs::prelude::*;

use crate::db::{BlockRow, PageRow};

/// A page is represented by a page object from the
/// database, and a tree of block objects with the data
/// for each node contained in the block_data vector
#[derive(Debug)]
pub struct Page {
    page_data: PageRow,
    block_data: Vec<BlockRow>,
    block_tree: Tree<usize>,
}

impl Page {
    pub fn new(page_row: PageRow, block_data: Vec<BlockRow>) -> Self {

        // create a mutable binding for the block data vector we're moving
        let mut block_vec = block_data;

        // create a dummy block for the tree root since a page may have multiple siblings at the top level
        block_vec.push(BlockRow { id: None, content: String::from(&page_row.title), parent_id: None, sibling_id: None, page_id: None });
        let dummy_index = block_vec.len() - 1;
        
        Page {
            page_data: page_row,
            block_data: block_vec,
            block_tree: Tree {
                value: dummy_index,
                children: Vec::new()
            }
        }
    }

    pub fn set_root_block(&mut self, root_block_id: Option<i64>) {
        self.page_data.root_block_id = root_block_id;
    }

    pub fn get_block_data_ref(&self) -> &Vec<BlockRow> {
        &self.block_data
    }

    fn get_siblings(&self, block_index: usize) -> Vec<usize> {
        let mut sibling_vector = Vec::new();

        let mut current_block = self.block_data.get(block_index).expect(&format!("block not found at index {}", block_index));

        while matches!(current_block.sibling_id, Option::Some(_)) {
            // find the sibling block's index
            let sibling_idx = self.block_data.iter().position(|block| {
                block.id == current_block.sibling_id
            }).expect(&format!("sibling not found with id {}", current_block.sibling_id.unwrap()));

            sibling_vector.push(sibling_idx);
            current_block = self.block_data.get(sibling_idx).unwrap();
        }

        return sibling_vector;
    }

    fn get_children(&self, block_index: usize) -> Vec<usize> {
        let mut child_vector = Vec::new();
        let current_block = &self.block_data[block_index];

        let children = self.block_data.iter().enumerate().filter(|(_index, block)| {
            block.parent_id == Some(current_block.id.unwrap())
        });

        for child in children {
            child_vector.push(child.0);
        }

        child_vector
    }

    /// from the root block build the tree based on parent and sibling
    /// ID fields of the blocks in block_data
    pub fn build_tree(&mut self) {
        
        let mut in_tree: Vec<bool> = Vec::with_capacity(self.block_data.len());
        in_tree.fill_with(|| {false});

        // initialize the in_tree vector and set the dummy root block as "in tree"
        in_tree[self.block_tree.value] = true;

        // find the root block in the data vector
        let root_block_index = self.block_data.iter()
        .position(|block| {
            block.id.is_some() && 
            block.id.unwrap() == self.page_data.root_block_id.unwrap()
        }).unwrap();

        // add the first block to the tree under the dummy root 
        self.block_tree.children.push(Tree {
            value: root_block_index,
            children: Vec::new()
        });

        in_tree[root_block_index] = true;

        let siblings = self.get_siblings(root_block_index);

        let mut sibling_nodes: Vec<Tree<usize>> = siblings.into_iter().map(|sibling| {
            Tree {
                value: sibling,
                children: Vec::new()
            }
        }).collect();

        // get the siblings of the first child, and add them to children
        self.block_tree.children.append(&mut sibling_nodes);

        let children = self.block_tree.children.iter();

        for child in children {
            in_tree[child.value] = true;
        }

        // tree is ready to be built down iteratively
        // TODO this would be a good place for multithreading, handing each subtree to a thread

    }
}