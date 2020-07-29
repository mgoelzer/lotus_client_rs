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

pub struct MsgPiecePayload {
    pub msg_cid: String,
    pub piece_cid: String,
    pub payload_cid: String,
}

impl MsgPiecePayload {
    pub fn new(msg_cid: &str, piece_cid: &str, payload_cid: &str) -> MsgPiecePayload {
        MsgPiecePayload{
            msg_cid: msg_cid.to_owned(),
            piece_cid: piece_cid.to_owned(),
            payload_cid: payload_cid.to_owned(),
        }
    }
}

pub fn lookup_cid(connection: &sqlite3::Connection, cid: &str) -> Option<MsgPiecePayload> {
    // Cursor select
    let mut cursor = connection
            .prepare("SELECT msg_cid, piece_cid, payload_cid FROM piece_payload WHERE piece_cid = ?1 OR payload_cid = ?1 LIMIT 1")
            .unwrap()
            .cursor();

    cursor.bind(&[  sqlite3::Value::String(cid.to_string())  ]).unwrap();
    
    if let Some(row) = cursor.next().unwrap() {
        println!("msg_cid = {}, piece_cid = {}, payload_cid = {}", 
                row[0].as_string().unwrap(), 
                row[1].as_string().unwrap(), 
                row[2].as_string().unwrap() );
        Some(MsgPiecePayload::new(row[0].as_string().unwrap(), 
                                  row[1].as_string().unwrap(),
                                  row[2].as_string().unwrap()))
    } else {
        None
    }
}
