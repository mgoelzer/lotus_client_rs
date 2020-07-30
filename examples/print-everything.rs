use lotus_client_rs::blockanalyzer::{Message,iterate_over_blockchain};
use env_logger;

fn main() {
    env_logger::Builder::from_default_env().format_timestamp(None).init();
    
    // Set up API object and test connection
    let api = lotus_client_rs::api::ApiClient::new("http://lotus1:1234/rpc/v0");
    assert!(api.check_endpoint_connection());
    
    // Define callbacks to print each message within each block
    let on_start_new_tipset = |height:u64,_blocks:&Vec<String>| {
        println!("Height {}",height);
    };
    let on_start_new_block = |blkcid:&str| {
        println!("  Block: {}",blkcid);
    };
    let on_found_new_message = |msg_cid:&str, msg:&Message| {
        println!("\n--- message {} ---\n{}{}", msg_cid, msg, "-".to_string().repeat(80));
    };

    // Run iterate_over_blockchain with our callbacks on the first few blocks
    iterate_over_blockchain(0, 5, &api, 
        Some(on_start_new_tipset),
        Some(on_start_new_block),
        None,
        Some(on_found_new_message),
        None,
        None
    );
}