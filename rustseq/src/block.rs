/// a BlockRow is a direct translation of a block row from the 
/// database. It will be used to create a tree of Block objects
/// in memory.
pub struct BlockRow {
    id: u32,
    content: String,

    parent_id: Option<u32>,
    sibling_id: Option<u32>,
}

impl BlockRow {
    pub fn new(id: u32, content: &str, parent_id: Option<u32>, sibling_id: Option<u32>) -> Self {
        BlockRow {
            id: id,
            content: String::from(content),
            parent_id: parent_id,
            sibling_id: sibling_id
        }
    }
}

#[derive(Debug)]
pub struct Block<'a, 'b> {
    id: u32,
    content: String,

    parent: Option<&'a Block<'a,'b>>,
    next_sibling: Option<&'b Block<'a,'b>>
}

impl<'a, 'b> Block<'a, 'b> {
    pub fn new(id: u32, content: &str) -> Self {
        Block {
            id: id,
            content: String::from(content),
            parent: Option::None,
            next_sibling: Option::None
        }
    }

    pub fn set_parent(&mut self, parent: &'a Block) {
        self.parent = Option::Some(parent);
    }

    pub fn set_sibling(&mut self, sibling: &'b Block) {
        self.next_sibling = Option::Some(sibling);
    }
}