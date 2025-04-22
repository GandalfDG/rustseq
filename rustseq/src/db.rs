use rusqlite;

use crate::block::Block;

pub struct DB {
    connection: rusqlite::Connection
}

impl DB {
    pub fn connect(path: &str) -> Self {
        let connection = rusqlite::Connection::open(path).expect("failed to open DB");
        DB {
            connection: connection
        }
    }

    pub fn create_tables(&self) {
        self.connection.execute(
            "CREATE TABLE blocks (
            id	INTEGER NOT NULL UNIQUE,
            content	TEXT,
            parent	INTEGER,
            next_sibling	INTEGER,
            PRIMARY KEY(id AUTOINCREMENT),
            FOREIGN KEY(next_sibling) REFERENCES blocks(id) ON DELETE SET NULL,
            FOREIGN KEY(parent) REFERENCES blocks(id) ON DELETE SET NULL);",
            ()).expect("failed to create table");
    }

    pub fn store_block(&self, block: &Block) {
        let block_row = block.as_block_row();

        let mut statement = self.connection.prepare("INSERT INTO blocks VALUES (?1, ?2, ?3, ?4);").expect("bad SQL string");
        statement.execute((block_row.id, block_row.content, block_row.parent_id, block_row.sibling_id));
    }
}

/// a BlockRow is a direct translation of a block row from the 
/// database. It will be used to create a tree of Block objects
/// in memory.
#[derive(Debug)]
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
