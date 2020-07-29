use lotus_client_rs::blockanalyzer::{Message,iterate_over_blockchain};
use lotus_client_rs::cbor::deal_proposal::decode_storage_deal;

use env_logger;
use log;
mod cli;
mod db;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Response, Server};
use hyper::http::StatusCode;
use std::sync::{Arc, Mutex};

fn main() {
    env_logger::Builder::from_default_env().format_timestamp(None).init();

    // Get CLI and config.toml args
    let exec_params = cli::parse_configuration();

    // Set up API object and test connection
    let api = lotus_client_rs::api::ApiClient::new(&exec_params.endpoint);
    assert!(api.check_endpoint_connection());
    
    // Define the callbacks
    let on_start_new_tipset = |height:u64,_blocks:&Vec<String>| {
        // add a row to tipset_processing_status table (cols: tipset_height, reported_starting, reported_ending)
        println!("Tipset: {}", height);
        let connection = db::get_db_connection();
        db::mark_height(&connection, height,false);
    };
    let on_complete_tipset = |height:u64| {
        // update tipset_processing_status.reported_ending
        println!("Completed tipset: {}", height);
        let connection = db::get_db_connection();
        db::mark_height(&connection, height,true);
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

    // Search for deal proposals (with Payload CID <-> Piece CID mappings)
    let on_found_new_message = |msg_cid:&str, msg:&Message| {
        // store it in messages (cols correspond to message, but indexes on all columns)
        log::info!("cid = '{}'\n\nmsg={:?}",msg_cid,msg);

        if msg.method=="4" && msg.to=="t05" {
            println!("Found a dealproposal:");
            println!("    cid = '{}'\n    msg={:?}\n",msg_cid,msg);
            if let Some(decoded_params) = decode_storage_deal(&msg.params) {
                println!("    decoded params = '{:?}'\n",decoded_params);
                println!("        piece_cid as str = '{}'\n",decoded_params.get_piece_cid_as_str());
    
                // Persist tuple to database
                let conn = db::get_db_connection();
                db::insert_piece_and_payload_cids(&conn,msg_cid,&decoded_params.get_piece_cid_as_str(),&decoded_params.label);
            } else {
                log::error!("on_found_new_message:  could not decode params for {}",msg_cid);
            }
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

    // TODO - this should start before we start looping over iterate_over_blockchain()
    // Start the HTTP interface on a background thread
    await_server();
}

#[tokio::main]
pub async fn await_server() {
    let addr = ([0, 0, 0, 0], 4500).into();

    // Shared threadsafe db connection
    let lock_conn = Arc::new(Mutex::new(db::get_db_connection()));

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
                            let msg_piece_payload : Option<db::MsgPiecePayload>;

                            {
                                // Get the db connection
                                let connection = &*lock_conn.lock().unwrap();
            
                                // Try to lookup cit
                                msg_piece_payload = db::lookup_cid(connection, cid_to_lookup);
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

