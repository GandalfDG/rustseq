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
        self.connection.execute_batch(
            "CREATE TABLE blocks (
            id	INTEGER NOT NULL UNIQUE,
            content	TEXT,
            parent	INTEGER,
            next_sibling	INTEGER,
            page INTEGER,
            PRIMARY KEY(id AUTOINCREMENT),
            FOREIGN KEY(next_sibling) REFERENCES blocks(id) ON DELETE SET NULL,
            FOREIGN KEY(parent) REFERENCES blocks(id) ON DELETE SET NULL
            FOREIGN KEY(page) REFERENCES pages(id) ON DELETE SET NULL);
            
            CREATE TABLE pages (
            id INTEGER NOT NULL UNIQUE,
            title TEXT,
            first_block INTEGER,
            PRIMARY KEY(id AUTOINCREMENT));
            ").expect("failed to create table");
    }

    /// insert a new block into the database, and get back its ID
    pub fn insert_block(&mut self, new_block: &mut BlockRow) -> Result<i64, rusqlite::Error> {

        let transaction = self.connection.transaction()?;

        transaction.execute("INSERT INTO blocks (content, parent, next_sibling, page) VALUES (?1, ?2, ?3, ?4);", 
                            (&new_block.content, &new_block.parent_id, &new_block.sibling_id, &new_block.page_id))?;

        new_block.id = Some(transaction.last_insert_rowid());
        transaction.commit()?;

        Ok(new_block.id.expect("no block ID"))  
    }

    /// insert a new page into the database, and get back its ID
    pub fn insert_page(&mut self, new_page: &mut PageRow) -> Result<i64, rusqlite::Error> {
        let transaction = self.connection.transaction()?;

        transaction.execute("INSERT INTO pages (title, first_block) VALUES (?1, ?2);",
                            (&new_page.title, &new_page.root_block_id))?;

        new_page.id = Some(transaction.last_insert_rowid());

        transaction.commit()?;
        Ok(new_page.id.expect("no page ID"))
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

    pub page_id: Option<i64>
}

pub struct PageRow {
   pub id: Option<i64>,
   pub title: String,

   pub root_block_id: Option<u32> 
}

