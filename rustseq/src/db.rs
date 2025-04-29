use std::result;

use rusqlite;

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

    pub fn update_block(&mut self, updated_block: &BlockRow) -> Result<(), rusqlite::Error> {
        let transaction = self.connection.transaction()?;

        transaction.execute("UPDATE blocks SET content=?1, parent=?2, next_sibling=?3, page=?4 WHERE id=?5",
            (&updated_block.content, updated_block.parent_id, updated_block.sibling_id, updated_block.page_id, updated_block.id))?;

        transaction.commit()?;

        return Ok(());
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

    pub fn update_page(&mut self, updated_page: &PageRow) -> Result<(), rusqlite::Error> {
        let transaction = self.connection.transaction()?;

        transaction.execute("UPDATE blocks SET title=?1, first_block=?2 WHERE id=?3",
            (&updated_page.title, &updated_page.root_block_id))?;

        transaction.commit()?;

        return Ok(());
    }

    pub fn get_page_blocks(&mut self, page: &PageRow) -> Result<Vec<BlockRow>, rusqlite::Error> {
        let transaction = self.connection.transaction()?;

        let mut statement = transaction.prepare("SELECT * FROM blocks WHERE page=?1")?;

        let rows = statement.query_map((page.id.unwrap(),), |row| {
            Ok(BlockRow{
                id: row.get(0)?,
                content: row.get(1)?,
                parent_id: row.get(2)?,
                sibling_id: row.get(3)?,
                page_id: row.get(4)?
            })
        })?;

        let mut row_block_vector = Vec::new();

        for row in rows {
            row_block_vector.push(row.unwrap());
        }

        return Ok(row_block_vector);

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

impl BlockRow {
    pub fn new(content: &str, parent_id: Option<i64>, sibling_id: Option<i64>, page_id: Option<i64>) -> Self {
        BlockRow { id: None, content: String::from(content), parent_id: parent_id, sibling_id: sibling_id, page_id: page_id }
    }
}

#[derive(Debug)]
pub struct PageRow {
   pub id: Option<i64>,
   pub title: String,

   pub root_block_id: Option<i64> 
}

