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
}