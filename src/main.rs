use std::collections::HashMap;

#[derive(Debug, Clone)]
struct BlsAggregateSignature {
    type_num: i64,
    data: String,
}
impl BlsAggregateSignature {
    fn new(type_num: i64, data: &str) -> BlsAggregateSignature {
        BlsAggregateSignature{
            type_num: type_num,
            data: String::from(data),
        }
    }
    fn copy(self: &BlsAggregateSignature) -> BlsAggregateSignature {
        BlsAggregateSignature::new(self.type_num, &self.data)
    }
}

#[derive(Debug, Clone)]
struct SecpkSignature {
    type_num: i64,
    data: String,
}
impl SecpkSignature {
    fn new(type_num: i64, data: &str) -> SecpkSignature {
        SecpkSignature{
            type_num: type_num,
            data: String::from(data),
        }
    }
}

#[derive(Debug, Clone)]
enum MessageTypeFlag {
    Unknown,
    BlsMessage(BlsAggregateSignature),
    SecpMessage(SecpkSignature),
}

#[derive(Debug, Clone)]
struct ReceiptFields { 
    exit_code: i64, 
    ret: String, 
    gas_used: u64, 
}

#[derive(Debug, Clone)]
enum ReceiptStatus {
    Receipt(ReceiptFields),
    #[allow(dead_code)] NoReceipt,
}

#[derive(Debug, Clone)]
struct Message {
    msg_type: MessageTypeFlag,
    version: u64, 
    to: String,
    from: String,
    nonce: u64,
    value: String,
    gas_price: String,
    gas_limit: u64,
    method: String,
    params: String,
    receipt: ReceiptStatus,
}

mod api;

fn get_tipset_by_height(height: u64) -> Vec<String> {
    let result : jsonrpsee::common::JsonValue = api::chain_get_tipset_by_height(height);
    let mut ret : Vec<String> = vec!();

    let mut i = 0;
    loop {
        let json_path = format!("/Cids/{}/~1",i);
        if let Some(cid) = result.pointer(&json_path) {
            let mut cid_str = cid.to_string();
            if cid_str.len()>2 {
                cid_str = cid_str[1..cid_str.len()-1].to_string();
            }
            //println!("cid_str = '{}'",cid_str);
            ret.push(cid_str);
        } else {
            break;
        }
        i += 1;
    }

    ret
}

fn get_current_tipset_height() -> u64 {
    let result : jsonrpsee::common::JsonValue = api::chain_head();
    let mut i = 0;
    let mut max_height = 0;
    loop {
        let json_path = format!("/Blocks/{}/Height",i);
        if let Some(height) = result.pointer(&json_path) {
            //println!("height = {}",height);
            let height_i32 = height.to_string().parse::<i32>().unwrap();
            if height_i32 > max_height {
                max_height = height_i32;
            }
        } else {
            break;
        }
        i += 1;
    }
    max_height as u64
}

fn parse_receipt_fields(receipt_jsonval: &jsonrpsee::common::JsonValue) -> (i64,String,u64) {
        let mut receipt_exit_code: i64 = -1;
        let mut receipt_return: String = "".to_string();
        let mut receipt_gas_used: u64 = 0;

        let json_path = format!("/ExitCode");
        if let Some(exitcode_jsonval) = receipt_jsonval.pointer(&json_path) {
            if let Some(exitcode_i64) = exitcode_jsonval.as_i64() {
                receipt_exit_code = exitcode_i64;
            }
        }

        let json_path = format!("/Return");
        if let Some(return_jsonval) = receipt_jsonval.pointer(&json_path) {
            receipt_return = return_jsonval.to_string();
            if receipt_return.len()>2 {
                receipt_return = receipt_return[1..receipt_return.len()-1].to_string();
            }
        }

        let json_path = format!("/GasUsed");
        if let Some(receipt_gas_used_jsonval) = receipt_jsonval.pointer(&json_path) {
            if let Some(gas_used_u64) = receipt_gas_used_jsonval.as_u64() {
                receipt_gas_used = gas_used_u64;
            }
        }

        (receipt_exit_code,receipt_return,receipt_gas_used)
}

fn parse_msg_fields(msg_jsonval : &jsonrpsee::common::JsonValue) -> (u64,String,String,u64,String,String,u64,String,String) {
    let mut version_u64 : u64 = 0;
    let mut to_str : String = "".to_string();
    let mut from_str : String = "".to_string();
    let mut nonce_u64 : u64 = 0;
    let mut value_str : String = "0".to_string();
    let mut gas_price_str : String = "0".to_string();
    let mut gas_limit_u64 : u64 = 0;
    let mut method_str : String = "0".to_string();
    let mut params_str : String = "".to_string();

    let json_path = format!("/Version");
    if let Some(version_jsonval) = msg_jsonval.pointer(&json_path) {
        version_u64 = match version_jsonval.as_u64() {
            Some(n) => { n },
            None => { 0 },
        }
    }

    let json_path = format!("/To");
    if let Some(to_jsonval) = msg_jsonval.pointer(&json_path) {
        to_str = to_jsonval.to_string();
        if to_str.len()>2 {
            to_str = to_str[1..to_str.len()-1].to_string();
        }
    }

    let json_path = format!("/From");
    if let Some(from_jsonval) = msg_jsonval.pointer(&json_path) {
        from_str = from_jsonval.to_string();
        if from_str.len()>2 {
            from_str = from_str[1..from_str.len()-1].to_string();
        }
    }

    let json_path = format!("/Nonce");
    if let Some(nonce_jsonval) = msg_jsonval.pointer(&json_path) {
        nonce_u64 = match nonce_jsonval.as_u64() {
            Some(n) => { n },
            None => { 0 },
        }
    }

    let json_path = format!("/Value");
    if let Some(value_jsonval) = msg_jsonval.pointer(&json_path) {
        value_str = value_jsonval.to_string();
        if value_str.len()>2 {
            value_str = value_str[1..value_str.len()-1].to_string();
        }
    }

    let json_path = format!("/GasLimit");
    if let Some(gas_limit_jsonval) = msg_jsonval.pointer(&json_path) {
        gas_limit_u64 = match gas_limit_jsonval.as_u64() {
            Some(n) => { n },
            None => { 0 },
        }
    }

    let json_path = format!("/GasPrice");
    if let Some(gas_price_jsonval) = msg_jsonval.pointer(&json_path) {
        gas_price_str = gas_price_jsonval.to_string();
        if gas_price_str.len()>2 {
            gas_price_str = gas_price_str[1..gas_price_str.len()-1].to_string();
        }
    }

    let json_path = format!("/Method");
    if let Some(method_jsonval) = msg_jsonval.pointer(&json_path) {
        method_str = method_jsonval.to_string();
        if method_str.len()>2 {
            method_str = method_str[1..method_str.len()-1].to_string();
        }
    }

    let json_path = format!("/Params");
    if let Some(params_jsonval) = msg_jsonval.pointer(&json_path) {
        params_str = params_jsonval.to_string();
        if params_str.len()>2 {
            params_str = params_str[1..params_str.len()-1].to_string();
        }
    }

    (version_u64, to_str, from_str, nonce_u64, value_str, gas_price_str, gas_limit_u64, method_str, params_str)
}

// combines the results of Filecoin.ChainGetParentMessages and .ChainGetParentReceipts
// to provide a tuple of {msg_cid, (message, receipt)}
fn iterate_parents_of_block(block_cid: &str, 
    msg_type_by_cid: &mut HashMap<String, MessageTypeFlag>, 
    msgs: &mut HashMap<String, Message>, 
    on_each_msgcid_msg_receipt_tuple: fn( msg_cid: &str,  msg: &Message) )
{
    let parent_msgs_jsonval : jsonrpsee::common::JsonValue = api::chain_get_parent_messages(block_cid);
    let parent_receipts_jsonval : jsonrpsee::common::JsonValue = api::chain_get_parent_receipts(block_cid);
    let mut i : u32 = 0;
    let mut consumed_all_cid_msg_pairs = false;
    loop {
        let mut cid_str : String = "".to_string();

        let mut version_u64 : u64 = 0;
        let mut to_str : String = "".to_string();
        let mut from_str : String = "".to_string();
        let mut nonce_u64 : u64 = 0;
        let mut value_str : String = "0".to_string();
        let mut gas_price_str : String = "0".to_string();
        let mut gas_limit_u64 : u64 = 0;
        let mut method_str : String = "0".to_string();
        let mut params_str : String = "".to_string();  
        
        let mut receipt_exit_code: i64 = -1;
        let mut receipt_return: String = "".to_string();
        let mut receipt_gas_used: u64 = 0;

        let cid_msg_json_path = format!("/{}",i);
        if let Some(cid_msg_jsonval) = parent_msgs_jsonval.pointer(&cid_msg_json_path) {
            let cid_msg_json_str = cid_msg_jsonval.to_string();

            let json_path = format!("/Cid/~1");
            if let Some(cid_jsonval) = cid_msg_jsonval.pointer(&json_path) {
                cid_str = cid_jsonval.to_string();
                cid_str = cid_str[1..cid_str.len()-1].to_string();
                //println!(">>>\ncid_str={}\n<<<\n",cid_str);
            } else {
                eprintln!("error: failed on cid_str extraction from this json blob:\n>>>>>\n'{}'\n<<<<<\n",cid_msg_json_str);
                break;
            }

            let json_path = format!("/Message");
            if let Some(msg_jsonval) = cid_msg_jsonval.pointer(&json_path) {
                let msg_fields = parse_msg_fields(msg_jsonval);
                version_u64 = msg_fields.0;
                to_str = msg_fields.1;
                from_str = msg_fields.2;
                nonce_u64 = msg_fields.3;
                value_str = msg_fields.4;
                gas_price_str = msg_fields.5;
                gas_limit_u64 = msg_fields.6;
                method_str = msg_fields.7;
                params_str = msg_fields.8;
            } else {
                eprintln!("error: failed on msg_jsonval extraction from this json blob:\n>>>>>\n'{}'\n<<<<<\n",cid_msg_json_str);
                break;
            }
        } else {
            consumed_all_cid_msg_pairs = true;
        }

        let receipt_json_path = format!("/{}",i);
        if let Some(receipt_jsonval) = parent_receipts_jsonval.pointer(&receipt_json_path) {
            let fields = parse_receipt_fields(receipt_jsonval);
            receipt_exit_code = fields.0;
            receipt_return = fields.1;
            receipt_gas_used = fields.2;
            assert_ne!(consumed_all_cid_msg_pairs,true,"All (cid,msg) pairs consumed but there is >= 1 unconsumed receipt: mismatch");
        } else {
            assert_eq!(consumed_all_cid_msg_pairs,true,"All (cid,msg) pairs not consumed yet but no remaining receipts: mismatch");
            break;
        }

        // Try to get the msg_type_flag from the head blocks previously examined, if possible
        let mut msg_type_flag = MessageTypeFlag::Unknown;
        if msg_type_by_cid.contains_key(&cid_str) {
            if let Some(mtf) = msg_type_by_cid.remove(&cid_str) {
                msg_type_flag = mtf;
            }
        }

        // Make the message struct
        let msg : Message = Message{
            msg_type: msg_type_flag,
            version: version_u64,
            to: to_str,
            from: from_str,
            nonce: nonce_u64,
            value: value_str,
            gas_price: gas_price_str,
            gas_limit: gas_limit_u64,
            method: method_str,
            params: params_str,
            receipt: ReceiptStatus::Receipt(ReceiptFields{
                exit_code: receipt_exit_code,
                ret: receipt_return,
                gas_used: receipt_gas_used,
            })
        };

        on_each_msgcid_msg_receipt_tuple(&cid_str, &msg);
        //println!("msg: {:?}",msg);
        msgs.insert(cid_str,msg);

        i += 1;
    } // loop
}

// We iterate over the messasges in a block in order to add them to a list of messages we expect to 
// see soon as parents (when they will have receipts).  For now we just store the msg_cid, the signature
// type and the actual signature.
//
// Gets all the json back from api::chain_get_block_messages then
// acts once one each {MsgCid,Message} pair found.
fn iterate_messages_for_block(block_cid: &str, 
    msg_type_by_cid: &mut HashMap<String, MessageTypeFlag>, 
    on_each_message: fn(msg_cid: &str, msg_type: &MessageTypeFlag)) 
{    
    // get block header and extract BLSAggregate from it
    let block_hdrs_jsonval : jsonrpsee::common::JsonValue = api::chain_get_block(block_cid);
    let bls_aggregate_type_num : i64;
    let mut bls_aggregate_data_str : String;
    if let Some(bls_aggregate_jsonval) = block_hdrs_jsonval.pointer("/BLSAggregate") {
        // TODO:  bls_aggregate_type should be a uint, not a str
        if let Some(bls_aggregate_type_jsonval) = bls_aggregate_jsonval.pointer("/Type") {
            bls_aggregate_type_num = match bls_aggregate_type_jsonval.as_i64() {
                Some(n) => { n },
                None => { -1 },
            }
        } else {
            bls_aggregate_type_num = -1;
        }
        if let Some(bls_aggregate_data_jsonval) = bls_aggregate_jsonval.pointer("/Data") {
            bls_aggregate_data_str = bls_aggregate_data_jsonval.to_string();
            bls_aggregate_data_str = bls_aggregate_data_str[1..bls_aggregate_data_str.len()-1].to_string();
        } else {
            bls_aggregate_data_str = "".to_string();
        }
    } else {
        bls_aggregate_type_num = -1;
        bls_aggregate_data_str = "".to_string();
    }
    let bls_aggregate_signature : BlsAggregateSignature = BlsAggregateSignature::new(bls_aggregate_type_num, &bls_aggregate_data_str);
    //println!("bls_aggregate_signature:  {:?}",bls_aggregate_signature);


    // Build vector of all cids in order
    let block_msgs_jsonval : jsonrpsee::common::JsonValue = api::chain_get_block_messages(block_cid);
    let mut msg_cid : String;
    let mut vd_msg_cids : std::collections::VecDeque<String> = 
        std::collections::VecDeque::new();
    let mut i : i32 = 0;
    loop {
        let json_path = format!("/Cids/{}/~1",i);
        if let Some(msg_cid_jsonval) = block_msgs_jsonval.pointer(&json_path) {
            msg_cid = msg_cid_jsonval.to_string();
            msg_cid = msg_cid[1..msg_cid.len()-1].to_string();
            //println!("msg_cid='{}'",msg_cid);
            vd_msg_cids.push_back(msg_cid);
        } else {
            break;
        }
        i += 1;
    }


    // Loop over each bls message
    i = 0;
    loop {
        let msg_type = MessageTypeFlag::BlsMessage(bls_aggregate_signature.copy());
        let bls_msg_json_path = format!("/BlsMessages/{}",i);
        if let Some(_bls_msg_jsonval) = block_msgs_jsonval.pointer(&bls_msg_json_path) {
            if let Some(next_msg_cid) = vd_msg_cids.pop_front() {
                //println!("\n>>>>\n{}, {:?}\n<<<<\n",next_msg_cid,bls_aggregate_signature);
                on_each_message(&next_msg_cid,&msg_type);
                msg_type_by_cid.insert(next_msg_cid, msg_type); 
            } else {
                assert!(false,"vd_msg_cids was empty prematurely: mismatch between number of message CIDs and number of BlsMessages");
            }
        } else {
            break;
        }
        i += 1;
    }


    // Loop over each secpk message
    i = 0;
    loop {
        let secpk_signature : SecpkSignature;
        let secpk_msg_json_path = format!("/SecpkMessages/{}",i);
        if let Some(secpk_msg_jsonval) = block_msgs_jsonval.pointer(&secpk_msg_json_path) {
            if let Some(next_msg_cid) = vd_msg_cids.pop_front() {
                let mut secp_signature_type_num : i64 = -1;
                let mut secp_signature_data_str : String = "".to_string();
                if let Some(secpk_signature_jsonval) = secpk_msg_jsonval.pointer("/Signature") {
                    if let Some(secpk_signature_type_jsonval) = secpk_signature_jsonval.pointer("/Type") {
                        secp_signature_type_num = match secpk_signature_type_jsonval.as_i64() {
                            Some(n)=>{n},
                            None=>{-1},
                        }
                    }
                    if let Some(secpk_signature_data_jsonval) = secpk_signature_jsonval.pointer("/Data") {
                        secp_signature_data_str = secpk_signature_data_jsonval.to_string();
                        if secp_signature_data_str.len()>2 {
                            secp_signature_data_str = secp_signature_data_str[1..secp_signature_data_str.len()-1].to_string();
                        }
                    }
                }
                secpk_signature = SecpkSignature::new(secp_signature_type_num, &secp_signature_data_str);
                let msg_type = MessageTypeFlag::SecpMessage(secpk_signature);
                //println!("\n>>>>\n{}, {:?}\n<<<<\n",next_msg_cid,secpk_signature);
                on_each_message(&next_msg_cid,&msg_type);
                msg_type_by_cid.insert(next_msg_cid, msg_type);
            } else {
                assert!(false,"vd_msg_cids was empty prematurely: mismatch between number of message CIDs and number of BlsMessages+SecpkMessages");
            }
        } else {
            break;
        }
        i += 1;
    }

    // assert that no msg_cids remain in queue
    assert_eq!(vd_msg_cids.len(),0,"All CIDs must now be exactly consumed");
}

fn main() {
    //
    // Command line args:  --min=N and --max=N are bounds on the range of tip set heights to index
    //
    let min_cli_arg = "--min=";
    let max_cli_arg = "--max=";
    let mut min_height : u64 = 0;
    let mut max_height : u64 = 999999999; // an impossibly large height that will get trimmed by max TSH

    let args : Vec<String> = std::env::args().collect();
    for mut argv in args {
        if argv.starts_with(min_cli_arg) {
            argv = argv[min_cli_arg.len()..].to_string();
            if let Ok(n) = argv.parse::<u64>() {
                min_height = n;
            } 
        } else if argv.starts_with(max_cli_arg) {
            argv = argv[max_cli_arg.len()..].to_string();
            if let Ok(n) = argv.parse::<u64>() {
                max_height = n;
            } 
        }
    }

    let curr_tipset_height = get_current_tipset_height();
    //println!("current tipset height: {}",curr_tipset_height);

    //
    // In-memory store
    //
    let mut msgs = HashMap::new();
    let mut msg_type_by_cid = HashMap::new();

    //
    // Iterate over the range of heights
    //
    use std::cmp::{min,max};
    let mut i : u64 = max(min_height,0 as u64);
    println!("Iterating from height {} to {}",i,min(max_height,curr_tipset_height));
    loop {
        if i > min(max_height,get_current_tipset_height()) {
            break
        }

        for blk_cid in get_tipset_by_height(i) {
            println!("Height {} : blk_cid {}...",i,blk_cid);
            // For teting
            // blk_cid = "bafy2bzacedtdy7sawc42n2yraczgpvqxf6saejzrz4hvp25k5hytwmcky7cq4"

            //
            // First store up current block's messages (which do not have receipts yet) in msg_type_by_cid 
            //
            iterate_messages_for_block(&blk_cid, &mut msg_type_by_cid,
                |msg_cid, msg_type_flag| {
                    println!("block messages:  {} : {:?}\n",msg_cid,msg_type_flag);
            });

            // 
            // Then gather up parents with their corresponding receipts for long-term storage in index
            // 
            iterate_parents_of_block(&blk_cid, &mut msg_type_by_cid, &mut msgs,
                |msg_cid, msg| {
                    println!("parent messages:  {} : {:?}\n",msg_cid,msg);

                    // Check the msg_cid and msg struct for well-formedness.
                    // Save this msg_cid,msg,receipt tuple into index db if good.
                    // If problems, save this msg_cid to a list of problem cids
                    // to re-retrieve another time.
                    //
                    // TODO...
            });

            println!(">> {}       msgs.len",msgs.len());
            println!(">> {}       msg_type_by_cid.len",msg_type_by_cid.len());
    
        }

        i += 1;
    }


    ////////////////////// Plan below //////////////////////////////////////////////
    //
    // 1.  Kill any the warnings
    // 0.  Verift the first 10 height of lotus1
    // 0.  Simplify this file:
    //      - Factor out some of the repetitive jsonval -> string/u64 code
    // 0.  In the interest of making progress, trap Ctrl+C and persist to disk the largest fully completed
    //     tipset height so that we can pick up at that height+1 next time we start.
    // 2. For any block_cid that we fail on, note it in a failed blocks list and just continue on with the
    //     next block.
    // 1.  Add a callback to check whether a given msg_cid is already in our index.
    //     Can use this to make index updates faster since don't even try to store msg_cids if
    //     already in db.
    // 3.  On shutdown, persist the list of msg_id=>msg_type_flag without receipts yet, so we
    //     can resume with it next time.  And:
        // TODO:
        // - Create a struct MessageFields without msg_type or receipt
        // - Lifecycle of a Message:
        //    - Initially you only have some subset of {Cid,MessageFields,MessageTypeFlag,ReceiptStatus}
        //    and can save those partial states in a "partial Messages" collection somewhere
        //    - When you have all the components, then you construct Message and save it in a
        //    "complete Messages" collection somewhere
        //    - The completes can be serialized to disk and deserialized later.  Partials can
        //    be serialized just as a msg_cid that is known to be missing.
        //    - Indices are constructed to allow fast lookup of complete Message structs 
        //        - e.g., lookup of the Payload CID for a Piece CID and vice-versa (known message type)
        //        - e.g., lookup whether a payment channel exists for a pair of addresses
    // 4.  Support a mode where we are running backward in time from current max height, to curr max -1, 
    //     curr max -2, etc.  It means that instead of automatically putting current blocks into 
    //     msg_id=>msg_type_flag map, we need to check whether we already have that msg_id with a receipt
    //     and if so just need to fill in the Message struct's msg_type_flag field.
}
