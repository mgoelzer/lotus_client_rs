use std::collections::HashMap;
mod api;

//
// Macros to cut down on boilerplate code to convert JsonValue
// objects to the native String, i32, u64, etc., rust types.
// (These could be improved on a lot!)
//
macro_rules! json_val_to_i32 {
    ( $json_path:literal , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: i32 = $receiving_variable_default_value;
        let json_path = format!($json_path);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            if let Some(inner_val) = jsonval.as_i32() {
                $receiving_variable = inner_val;
            }
        }
    }
}

macro_rules! json_val_to_i64 {
    ( $json_path:literal , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: i64 = $receiving_variable_default_value;
        let json_path = format!($json_path);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            if let Some(inner_val) = jsonval.as_i64() {
                $receiving_variable = inner_val;
            }
        }
    }
}

macro_rules! json_val_to_string {
    ( $json_path:literal , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: String = $receiving_variable_default_value.to_string();
        let json_path = format!($json_path);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            $receiving_variable = jsonval.to_string();
            if $receiving_variable.len()>2 {
                $receiving_variable = $receiving_variable[1..$receiving_variable.len()-1].to_string();
            }
        }
    }
}

macro_rules! json_val_to_string_with_formatter {
    ( $json_path:literal # $arg0:ident , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: String = $receiving_variable_default_value.to_string();
        let json_path = format!($json_path, $arg0);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            $receiving_variable = jsonval.to_string();
            if $receiving_variable.len()>2 {
                $receiving_variable = $receiving_variable[1..$receiving_variable.len()-1].to_string();
            }
        }
    }
}

macro_rules! json_val_to_i32_with_formatter {
    ( $json_path:literal # $arg0:ident , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: i32 = $receiving_variable_default_value;
        let json_path = format!($json_path, $arg0);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            if let Ok(inner_val) = jsonval.to_string().parse::<i32>() {
                $receiving_variable = inner_val;
            }
        }
    }
}

macro_rules! json_val_to_u64 {
    ( $json_path:literal , $jsonval:ident, $receiving_variable:ident, $receiving_variable_default_value:literal ) => {
        #[allow(unused_mut)]
        let mut $receiving_variable: u64 = $receiving_variable_default_value;
        let json_path = format!($json_path);
        if let Some(jsonval) = $jsonval.pointer(&json_path) {
            if let Some(inner_val) = jsonval.as_u64() {
                $receiving_variable = inner_val;
            }
        }
    }
}

//
// Data structures for storing and processing blockchain messages
//

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
pub struct Message {
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

impl Message {
    pub fn new() -> Message {
        Message{
            msg_type: MessageTypeFlag::Unknown,
            version : 0,
            to : "".to_string(),
            from : "".to_string(),
            nonce : 0,
            value : "0".to_string(),
            gas_price : "0".to_string(),
            gas_limit : 0,
            method : "0".to_string(),
            params : "".to_string(),
            receipt: ReceiptStatus::NoReceipt,
        }
    }
}

#[derive(Debug, Clone)]
struct MessageBuilder {
    pub msg: Message,
}

impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder{
            msg: Message::new(),
        }
    }

    pub fn msg_fields<'a>(&'a mut self, 
        msg_jsonval: &jsonrpsee::common::JsonValue) -> &'a mut MessageBuilder 
    {
        json_val_to_u64!(    "/Version", msg_jsonval, version_u64,      0);
        json_val_to_string!( "/To",      msg_jsonval, to_str,          "");
        json_val_to_string!( "/From",    msg_jsonval, from_str,        "");
        json_val_to_u64!(    "/Nonce",   msg_jsonval, nonce_u64,        0);
        json_val_to_string!( "/Value",   msg_jsonval, value_str,      "0");
        json_val_to_u64!(    "/GasLimit",msg_jsonval, gas_limit_u64,    0);
        json_val_to_string!( "/GasPrice",msg_jsonval, gas_price_str,  "0");
        json_val_to_string!( "/Method",  msg_jsonval, method_str,     "0");
        json_val_to_string!( "/Params",  msg_jsonval, params_str,      "");
        self.msg.version   = version_u64;
        self.msg.to        = to_str;
        self.msg.from      = from_str;
        self.msg.nonce     = nonce_u64;
        self.msg.value     = value_str;
        self.msg.gas_limit = gas_limit_u64;
        self.msg.gas_price = gas_price_str;
        self.msg.method    = method_str;
        self.msg.params    = params_str;
        self
    }

    pub fn receipt_field<'a>(&'a mut self, receipt_jsonval: &jsonrpsee::common::JsonValue) -> &'a mut MessageBuilder {
        json_val_to_i64!(    "/ExitCode", receipt_jsonval, receipt_exit_code, -1);
        json_val_to_string!( "/Return",   receipt_jsonval, receipt_return,    "");
        json_val_to_u64!(    "/GasUsed",  receipt_jsonval, receipt_gas_used,   0);
        self.msg.receipt = ReceiptStatus::Receipt(ReceiptFields{
            exit_code: receipt_exit_code,
            ret: receipt_return,
            gas_used: receipt_gas_used,
        });
        self
    }

    pub fn msg_type<'a>(&'a mut self, msg_type_flag: MessageTypeFlag) -> &'a mut MessageBuilder {
        self.msg.msg_type = msg_type_flag;
        self
    }

    pub fn get(&mut self) -> Message {
        let mut alt_msg = Message::new();
        std::mem::swap(&mut self.msg, &mut alt_msg);
        alt_msg
    }
}


//
// Blockchain reading and indexing functions
//

fn get_tipset_by_height(height: u64) -> Vec<String> {
    let result : jsonrpsee::common::JsonValue = api::chain_get_tipset_by_height(height);
    let mut ret : Vec<String> = vec!();
    let mut cid_str : String;
    let mut i = 0;
    loop {
        json_val_to_string_with_formatter!( "/Cids/{}/~1"#i, result, cid_str, "");
        if cid_str != "" {
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
    let mut height_i32 : i32;
    let mut max_height = 0;
    loop {
        json_val_to_i32_with_formatter!("/Blocks/{}/Height"#i, result, height_i32, 0);
        if height_i32 > 0 {
            max_height = std::cmp::max(max_height,height_i32);
        } else {
            break;
        }
        i += 1;
    }
    max_height as u64
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
        let mut msg_builder = MessageBuilder::new();
        let mut msg_jsonval : &jsonrpsee::common::JsonValue = &jsonrpsee::common::JsonValue::Null;
        let mut receipt_jsonval : &jsonrpsee::common::JsonValue = &jsonrpsee::common::JsonValue::Null;

        let cid_msg_json_path = format!("/{}",i);
        if let Some(cid_msg_jsonval) = parent_msgs_jsonval.pointer(&cid_msg_json_path) {
            let cid_msg_json_str = cid_msg_jsonval.to_string();

            let json_path = format!("/Cid/~1");
            if let Some(cid_jsonval) = cid_msg_jsonval.pointer(&json_path) {
                cid_str = cid_jsonval.to_string();
                cid_str = cid_str[1..cid_str.len()-1].to_string();
            } else {
                eprintln!("error: failed on cid_str extraction from this json blob:\n>>>>>\n'{}'\n<<<<<\n",cid_msg_json_str);
                break;
            }

            let json_path = format!("/Message");
            if let Some(msg_jsonval_) = cid_msg_jsonval.pointer(&json_path) {
                msg_jsonval = msg_jsonval_;
            } else {
                eprintln!("error: failed on msg_jsonval extraction from this json blob:\n>>>>>\n'{}'\n<<<<<\n",cid_msg_json_str);
                break;
            }
        } else {
            consumed_all_cid_msg_pairs = true;
        }

        let receipt_json_path = format!("/{}",i);
        if let Some(receipt_jsonval_) = parent_receipts_jsonval.pointer(&receipt_json_path) {
            receipt_jsonval = receipt_jsonval_;
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
        let message : Message = msg_builder
                            .msg_fields(msg_jsonval)
                            .msg_type(msg_type_flag)
                            .receipt_field(receipt_jsonval)
                            .get();

        on_each_msgcid_msg_receipt_tuple(&cid_str, &message);
        msgs.insert(cid_str,message);

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

    let mut args : Vec<String> = std::env::args().collect();
    let progname = args.remove(0);
    let usage_and_exit = |ret| { 
        let path = std::path::Path::new(&progname);
        let progname = path.file_name().unwrap().to_string_lossy();
        println!("\nUSAGE:  {} [--min=N] [--max=N]\n", progname); 
        std::process::exit(ret); 
    };
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
        } else {
            eprintln!("command line argument '{}' was not expected",&argv);
            usage_and_exit(1);
        }
    }

    let mut curr_tipset_height = get_current_tipset_height();
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
        curr_tipset_height = get_current_tipset_height();
        if i > min(max_height,curr_tipset_height) {
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
