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

    /// insert a new block into the database, and get back its ID
    /// return a BlockRow result containing the ID
    pub fn create_block(&mut self, content: Option<&str>, parent_id: Option<i64>, sibling_id: Option<i64>) -> Result<BlockRow, rusqlite::Error> {
        let mut new_block = BlockRow::new(None, content.unwrap_or_default(), parent_id, sibling_id);

        let transaction = self.connection.transaction()?;

        transaction.execute("INSERT INTO blocks (content, parent, next_sibling) VALUES (?1, ?2, ?3);", 
                            (&new_block.content, &new_block.parent_id, &new_block.sibling_id))?;

        new_block.id = Some(transaction.last_insert_rowid());
        transaction.commit()?;

        Ok(new_block)  
    }
}

/// a BlockRow is a direct translation of a block row from the 
/// database. It will be used to create a tree of Block objects
/// in memory.
#[derive(Debug)]
pub struct BlockRow {
    pub id: Option<i64>,
    pub content: String,

    pub parent_id: Option<i64>,
    pub sibling_id: Option<i64>,
}

impl BlockRow {
    pub fn new(id: Option<i64>, content: &str, parent_id: Option<i64>, sibling_id: Option<i64>) -> Self {
        BlockRow {
            id: id,
            content: String::from(content),
            parent_id: parent_id,
            sibling_id: sibling_id
        }
    }
}

pub struct PageRow {
   id: u32,
   title: String,

   root_block_id: Option<u32> 
}

impl PageRow {
    pub fn new(id: u32, title: &str, root_block_id: Option<u32>) -> Self {
        PageRow {
            id: id,
            title: String::from(title),
            root_block_id: root_block_id
        }
    }
}
