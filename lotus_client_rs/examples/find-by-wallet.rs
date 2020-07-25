use lotus_client_rs::blockanalyzer::{Message,iterate_over_blockchain};

fn main() {
    let api = lotus_client_rs::api::ApiClient::new("http://lotus1:1234/rpc/v0");
    assert!(api.check_endpoint_connection());
    
    let on_height = |height:u64,_blocks:&Vec<String>| {
        if height % 10 == 0 {
            println!("(at height {}-{})",height,height+9);
        }
    };
    let on_msg = |msg_cid:&str, msg:&Message| {
        let find_addr = "t3wzcpwznw6dvl6x3beekspluvhdwh26h3tvmw5y2fychse7pr6xlsfmxuhsv6ki7r3pm6s7gxc65h52lgqfsa";
        if msg.from==find_addr || msg.to==find_addr {
            println!("Message {}:\n  From {}\n  To {}\n",msg_cid,msg.from,msg.to);
        }
    };
    iterate_over_blockchain(0, 50, &api, Some(on_height), None, None, Some(on_msg), None, None);
}