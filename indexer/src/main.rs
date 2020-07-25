use lotus_client_rs::blockanalyzer::{Message,iterate_over_blockchain};
use env_logger;
use log;
mod cli;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Response, Server};
use hyper::http::StatusCode;
use std::sync::{Arc, Mutex};

use sqlite3;

use std::convert::TryFrom;

fn get_db_connection() -> sqlite3::Connection {
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

fn create_tables(connection: &sqlite3::Connection) {
    connection
        .execute(
            "
            DROP TABLE IF EXISTS piece_payload;
            CREATE TABLE piece_payload (msg_cid TEXT, piece_cid TEXT, payload_cid TEXT);
            DROP TABLE IF EXISTS heights;
            CREATE TABLE heights (height INTEGER, completed INTEGER DEFAULT '0' NOT NULL);
            ",
        )
        .unwrap();
}

fn insert_piece_and_payload_cids(connection: &sqlite3::Connection, msg_cid: &str, piece_cid:&str, payload_cid:&str) {
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

fn mark_height(connection: &sqlite3::Connection, height: u64, is_completed: bool) {
    let int_completed = match is_completed {
        true => { 1 },
        false => { 0 },
    };
    let height_int64 : i64 = 0_i64 + i64::try_from(height).unwrap();

    // TODO: needs to mark not just whether the height was started and completed, but how to 
    // roll back in case program starts up in a state where a certain height is started but
    // not completed

    connection
        .execute(format!(
            "INSERT INTO heights (height, completed) VALUES ('{}', '{}')",
            height_int64,
            int_completed
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

fn lookup_cid(connection: &sqlite3::Connection, cid: &str) -> Option<MsgPiecePayload> {
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

fn main() {
    env_logger::Builder::from_default_env().format_timestamp(None).init();

    // Get CLI and config.toml args
    let exec_params = cli::parse_configuration();

    // Set up API object and test connection
    let api = lotus_client_rs::api::ApiClient::new(&exec_params.endpoint);
    assert!(api.check_endpoint_connection());
    
    ///////////////////////////////////////////////////

    // Define the callbacks
    let on_start_new_tipset = |_height:u64,_blocks:&Vec<String>| {
        // add a row to tipset_processing_status table (cols: tipset_height, reported_starting, reported_ending)
        println!("Tipset: {}", _height);
        let connection = get_db_connection();
        mark_height(&connection, _height,false);
    };
    let on_complete_tipset = |_height:u64| {
        // update tipset_processing_status.reported_ending
        println!("Completed tipset: {}", _height);
        let connection = get_db_connection();
        mark_height(&connection, _height,true);
    };

    let on_start_new_block = |_blkcid:&str| {
        // add a row to block_cids (cols:  id, block_cid)
        // add a row to blocks_processing (cols:  block_cid_id, reported_starting, reported_ending)
        println!("  Block: {}", _blkcid);
    };
    let on_block_complete = |_blkcid:&str| {
        // update blocks_processing.reported_ending
    };

    let on_found_new_message_cid = |_msg_cid:&str| {
        // No-op
        log::info!("cid = '{}'",_msg_cid);
    };

    let on_found_new_message = |_msg_cid:&str, _msg:&Message| {
        // store it in messages (cols correspond to message, but indexes on all columns)
        log::info!("cid = '{}'\n\nmsg={:?}",_msg_cid,_msg);

        if _msg.method=="4" && (_msg.from=="t3rbcytzb6t3vzeigkfi6oxg22iwpaeekaf7gbstovykkmskg2nglthnzpg777whuevo4loqu3vmthoxsvdkbq" || _msg.from=="t3schhtef3m5yshlop3ffwjk5lauaqryx2e67xayngbj5stvha6m6i5noqvtiswykqg5pel4wt5j6zlrbblxbq") {
            println!("    cid = '{}'\n    msg={:?}\n",_msg_cid,_msg);

            let conn = get_db_connection();
            insert_piece_and_payload_cids(&conn,_msg_cid,"88888888888888888888dasfadsfdsfdsfa","8888888dasfdsfadsfdksfladskfjaksdlflasdfaads");
        }
    };

    // Run executor with our callbacks
    iterate_over_blockchain(exec_params.min, exec_params.max, &api, 
        Some(on_start_new_tipset),
        Some(on_start_new_block),
        Some(on_found_new_message_cid),
        Some(on_found_new_message),
        Some(on_block_complete),
        Some(on_complete_tipset)
    );



    //////////////////////////////////////////////////
    
    await_server();
}



#[tokio::main]
pub async fn await_server() {
    let addr = ([0, 0, 0, 0], 4500).into();

    // For the most basic of state, we just share a counter, that increments
    // with each request, and we send its value back in the response.
    let lock_conn = Arc::new(Mutex::new(get_db_connection()));

    // Closure `make_service_fn` is run for each connection,
    // creating a 'service' to handle requests for that connection.
    let make_service = make_service_fn(move |_| {
        // While the state was moved into the make_service closure,
        // we need to clone it here because this closure is called
        // once for every connection.
        let lock_conn = lock_conn.clone();

        async move {
            // This is the `Service` that will handle the connection.
            // `service_fn` is a helper to convert a function that
            // returns a Response into a `Service`.
            Ok::<_, Error>(service_fn(move |req| {
                let body_string;
                let status_code;
                match req.uri().path() {
                    "/lookup-cid" => {
                        if let Some(uri_query_str) = req.uri().query() {
                            let cid_to_lookup = uri_query_str;
                            let msg_piece_payload : Option<MsgPiecePayload>;

                            {
                                // Get the db connection
                                let connection = &*lock_conn.lock().unwrap();
            
                                // Try to lookup cit
                                msg_piece_payload = lookup_cid(connection, cid_to_lookup);
                            }

                            body_string = match msg_piece_payload {
                                Some(mpp) => { 
                                    status_code = StatusCode::OK;
                                    format!("{{\"status\": \"success\", \"lookup_cid\": \"{}\", \"msg_cid\": \"{}\", \"piece_cid\": \"{}\", \"payload_cid\": \"{}\"}}", 
                                        cid_to_lookup, mpp.msg_cid, mpp.piece_cid, mpp.payload_cid)
                                },
                                None => {
                                    status_code = StatusCode::OK;
                                    format!("{{\"status\": \"failed\", \"message\": \"couldn't find CID '{}'\"}}", cid_to_lookup)
                                }
                            };
                        } else {
                            status_code = StatusCode::OK;
                            body_string = "{\"status\": \"failed\", \"message\": \"couldn't parse url query string\"}".to_owned();
                        }
                    },
                    _ => {
                        status_code = StatusCode::NOT_FOUND;
                        body_string = "".to_owned();
                    },
                }

                let mut response = Response::new(Body::from(body_string));
                *response.status_mut() = status_code;
                async move { Ok::<_, Error>(response) }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

