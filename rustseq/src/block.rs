#[derive(Debug)]
pub struct Block {
    content: String,

    parent: Option<&Block>,
    next_sibling: Option<&Block>
}

impl Block {
    pub fn new(content: &str) -> Self {
        Block {
            content: String::from(content),
            parent: Option::None,
            next_sibling: Option::None
        }
    }

    pub fn set_parent(&self, parent: &Block) {
        self.parent = Option::Some(parent);
    }

    pub fn set_sibling(&self, sibling: &Block) {
        self.next_sibling = Option::Some(sibling);
    }
}