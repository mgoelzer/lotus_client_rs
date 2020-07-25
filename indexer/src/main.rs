use lotus_client_rs;
use lotus_client_rs::blockanalyzer::Message;
use env_logger;
use log;
mod cli;

fn executor(exec_params : & cli::ExecutionParameters, api : &lotus_client_rs::api::ApiClient, 
    on_starting_new_tipset:     std::option::Option<fn(height: u64, blocks: &Vec<String>)>,
    on_starting_block:          std::option::Option<fn(blk_cid: &str)>,
    on_found_new_message_cid:   std::option::Option<fn(msg_cid: &str)>,
    on_found_new_message:       std::option::Option<fn(msg_cid: &str, msg: &Message)>,
    on_finished_block:          std::option::Option<fn(blk_cid: &str)>,
    on_finished_tipset:         std::option::Option<fn(height: u64)>) 
{
    use lotus_client_rs::blockanalyzer::BlockAnalyzer;
    use lotus_client_rs::blockanalyzer::{Tipsets,MaxTipsetHeight};

    log::info!("executor:  min={},max={},endpoint='{}'",exec_params.min,exec_params.max,exec_params.endpoint);

    //
    // Construct block analyzer
    //
    let mut block_analyzer = BlockAnalyzer::new(&api);

    //
    // Iterate over the range of heights
    //
    let mut curr_tipset_height = MaxTipsetHeight::new(&api).max_height;
    log::debug!("current largest tipset height: {})",curr_tipset_height);
    use std::cmp::{min,max};
    let mut i : u64 = max(exec_params.min,0 as u64);
    log::info!("Iterating from height {} to {}",i,min(exec_params.max,curr_tipset_height));
    loop {
        let ts_strings : Vec<String> = Tipsets::new(&api,i).collect();
        if let Some(f) = on_starting_new_tipset {
            f(i,&ts_strings);
        }
        for blk_cid in ts_strings {
            if let Some(f) = on_starting_block {
                f(&blk_cid);
            }

            log::info!("Height {} : blk_cid {}...",i,blk_cid);

            // Iterate complete messages referenced in this block, and cids of new messages first
            // appearing in this block.
            if on_found_new_message.is_some() || on_found_new_message_cid.is_some()
            {
                block_analyzer.iterate_over_all_messages_in_block(&blk_cid, on_found_new_message, 
                    on_found_new_message_cid);
            }

            if let Some(f) = on_finished_block {
                f(&blk_cid);
            }
        }

        if let Some(f) = on_finished_tipset {
            f(i);
        }

        //
        // Loop control
        //
        i += 1;
        if i > min(exec_params.max,curr_tipset_height) {
            curr_tipset_height = MaxTipsetHeight::new(&api).max_height;
            if i > min(exec_params.max,curr_tipset_height) {
                break
            }
        }
    }
}

fn main() {    
    env_logger::Builder::from_default_env().format_timestamp(None).init();

    // Get CLI and config.toml args
    let exec_params = cli::parse_configuration();

    // Set up API object and test connection
    let api = lotus_client_rs::api::ApiClient::new(&exec_params.endpoint);
    assert!(api.check_endpoint_connection());
    
    // Define the callbacks
    let on_start_new_tipset = |_height:u64,_blocks:&Vec<String>| {
        // add a row to tipset_processing_status table (cols: tipset_height, reported_starting, reported_ending)
    };
    let on_complete_tipset = |_height:u64| {
        // update tipset_processing_status.reported_ending
    };

    let on_start_new_block = |_blkcid:&str| {
        // add a row to block_cids (cols:  id, block_cid)
        // add a row to blocks_processing (cols:  block_cid_id, reported_starting, reported_ending)
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
    };

    // Run executor with our callbacks
    executor(&exec_params, &api, 
        Some(on_start_new_tipset),
        Some(on_start_new_block),
        Some(on_found_new_message_cid),
        Some(on_found_new_message),
        Some(on_block_complete),
        Some(on_complete_tipset)
    );
}

//1. Move all block analyzer stuff to another file
//3. Write the callback logic I want for demo (both simple and useful)
//4. Put the simple demo in the README.md as an example of what you can do
//      --> Simple demo could be to accumulate how much FC has been spend on storage vs retrieval
//      --> more complex may require postgres integration
//5. Add compile/run instructions in README.md


    ////////////////////// Ideas for improvement //////////////////////////////////////////////
    //
    // 2.  For any block_cid that we fail on, note it in a failed blocks list and just continue 
    //     on with the next block.
    // 3.  On shutdown, persist the list of msg_id=>msg_type_flag without receipts yet, so we
    //     can resume with it next time.  And:
    //    TODO:
    //    - Indices are constructed to allow fast lookup of complete Message structs 
    //        - e.g., lookup of the Payload CID for a Piece CID and vice-versa (known message type)
    //        - e.g., lookup whether a payment channel exists for a pair of addresses
    // 4.  Support a mode where we are running backward in time from current max height, to curr max -1, 
    //     curr max -2, etc.  It means that instead of automatically putting current blocks into 
    //     msg_id=>msg_type_flag map, we need to check whether we already have that msg_id with a receipt
    //     and if so just need to fill in the Message struct's msg_type_flag field.

