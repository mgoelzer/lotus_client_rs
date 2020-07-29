use lotus_client_rs::blockanalyzer::{Message,iterate_over_blockchain};
use env_logger;

fn main() {
    env_logger::Builder::from_default_env().format_timestamp(None).init();
    
    let api = lotus_client_rs::api::ApiClient::new("http://lotus1:1234/rpc/v0");
    assert!(api.check_endpoint_connection());
    
    let on_height = |height:u64,_blocks:&Vec<String>| {
        println!("Tipset height: {}",height);
    };
    let on_msg = |msg_cid:&str, msg:&Message| {
        let find_prefix = "t3";
        if msg.from.starts_with(&find_prefix) || msg.to.starts_with(&find_prefix) {
            println!("Message {}:\n  From {}\n  To {}\n",msg_cid,msg.from,msg.to);
        }
    };
    iterate_over_blockchain(0, 50, &api, Some(on_height), None, None, Some(on_msg), None, None);
}