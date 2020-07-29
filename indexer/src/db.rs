use sqlite3;
use crate::cli;
use std::convert::TryFrom;

pub fn get_db_connection() -> sqlite3::Connection {
    let mut db_pathbuf : std::path::PathBuf = match dirs::home_dir() {
        Some(path) => { path },
        None       => { std::path::PathBuf::from(".") },
    };
    db_pathbuf.push(cli::CONFIG_DIR);
    db_pathbuf.push("db.sqlite3");

    let db_path = db_pathbuf.as_path(); 
    let connection = sqlite3::open(db_path).unwrap();

    if ! db_path.exists() {
        create_tables(&connection);
    }
    
    connection
}

pub fn create_tables(connection: &sqlite3::Connection) {
    connection
        .execute(
            "
            DROP TABLE IF EXISTS piece_payload;
            CREATE TABLE piece_payload (msg_cid TEXT, piece_cid TEXT, payload_cid TEXT);
            DROP TABLE IF EXISTS heights;
            CREATE TABLE heights (id INTEGER PRIMARY KEY, height INTEGER, completed INTEGER DEFAULT '0' NOT NULL, UNIQUE(height));
            ",
        )
        .unwrap();
}

pub fn insert_piece_and_payload_cids(connection: &sqlite3::Connection, msg_cid: &str, piece_cid:&str, payload_cid:&str) {
    // Insert
    connection
        .execute(format!(
            "INSERT INTO piece_payload (msg_cid, piece_cid, payload_cid) VALUES ('{}', '{}', '{}')",
            msg_cid,
            piece_cid,
            payload_cid
        ))
        .unwrap();
}

pub fn mark_height(connection: &sqlite3::Connection, height: u64, is_completed: bool) {
    let height_int64 : i64 = 0_i64 + i64::try_from(height).unwrap();
    connection
        .execute(format!(
            "INSERT INTO heights (height, completed) VALUES ('{}', '{}') ON CONFLICT(height) DO UPDATE SET completed='{}';",
            height_int64,
            is_completed as i32,
            is_completed as i32
        ))
        .unwrap();
}