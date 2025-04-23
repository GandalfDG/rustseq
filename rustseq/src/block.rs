use crate::db::BlockRow;

#[derive(Debug)]
pub struct Block<'a, 'b> {
    id: i64,
    content: String,

    parent: Option<&'a Block<'a,'b>>,
    next_sibling: Option<&'b Block<'a,'b>>
}

impl<'a, 'b> Block<'a, 'b> {
    pub fn new(id: i64, content: &str) -> Self {
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

    pub fn as_block_row(&self) -> BlockRow {
        // parent ID or rusqlite::types::Null
        let parent_id = match self.parent {
            Some(parent_block) => Some(parent_block.id),
            None => None
        };

        let sibling_id = match self.next_sibling {
            Some(sibling_block) => Some(sibling_block.id),
            None => None
        };

        BlockRow {id: Some(self.id), content: String::from(&self.content), parent_id: parent_id, sibling_id: sibling_id, page_id: None}
    }
}