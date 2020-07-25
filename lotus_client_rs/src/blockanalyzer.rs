use log;
use crate::api;
use std::collections::HashMap;
// See https://github.com/rust-lang/rust/issues/57966 re why this is commented
//#[macro_use] use crate::macro;

#[derive(Debug, Clone)]
pub struct BlsAggregateSignature {
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
pub struct SecpkSignature {
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
pub enum MessageTypeFlag {
    Unknown,
    BlsMessage(BlsAggregateSignature),
    SecpMessage(SecpkSignature),
}

#[derive(Debug, Clone)]
pub struct ReceiptFields { 
    exit_code: i64, 
    ret: String, 
    gas_used: u64, 
}

#[derive(Debug, Clone)]
pub enum ReceiptStatus {
    Receipt(ReceiptFields),
    #[allow(dead_code)] NoReceipt,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub msg_type: MessageTypeFlag,
    pub version: u64, 
    pub to: String,
    pub from: String,
    pub nonce: u64,
    pub value: String,
    pub gas_price: String,
    pub gas_limit: u64,
    pub method: String,
    pub params: String,
    pub receipt: ReceiptStatus,
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
pub struct MessageBuilder {
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
        crate::json_val_to_u64!(    "/Version", msg_jsonval, version_u64,      0);
        crate::json_val_to_string!( "/To",      msg_jsonval, to_str,          "");
        crate::json_val_to_string!( "/From",    msg_jsonval, from_str,        "");
        crate::json_val_to_u64!(    "/Nonce",   msg_jsonval, nonce_u64,        0);
        crate::json_val_to_string!( "/Value",   msg_jsonval, value_str,      "0");
        crate::json_val_to_u64!(    "/GasLimit",msg_jsonval, gas_limit_u64,    0);
        crate::json_val_to_string!( "/GasPrice",msg_jsonval, gas_price_str,  "0");
        crate::json_val_to_string!( "/Method",  msg_jsonval, method_str,     "0");
        crate::json_val_to_string!( "/Params",  msg_jsonval, params_str,      "");
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
        crate::json_val_to_i64!(    "/ExitCode", receipt_jsonval, receipt_exit_code, -1);
        crate::json_val_to_string!( "/Return",   receipt_jsonval, receipt_return,    "");
        crate::json_val_to_u64!(    "/GasUsed",  receipt_jsonval, receipt_gas_used,   0);
        self.msg.receipt = ReceiptStatus::Receipt(ReceiptFields{
            exit_code: receipt_exit_code,
            ret:       receipt_return,
            gas_used:  receipt_gas_used,
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

pub struct BlockAnalyzer<'a> {
    api : &'a api::ApiClient,
    pub incomplete_msg_cache : HashMap<String,MessageTypeFlag>,
}

impl<'a> BlockAnalyzer<'_> {
    pub fn new(api : &'a api::ApiClient) -> BlockAnalyzer {
        BlockAnalyzer{
            api : api,
            incomplete_msg_cache : HashMap::new(),
        }
    }

    // combines the results of Filecoin.ChainGetParentMessages and .ChainGetParentReceipts
    // to create tuple of {msg_cid, message, receipt)}, which is the fully formed message type.
    fn iterate_over_complete_messages_in_block(&mut self, block_cid: &str, 
        each_complete_message: fn(msg_cid: &str,  msg: &Message))
    {
        let parent_msgs_jsonval : jsonrpsee::common::JsonValue = self.api.chain_get_parent_messages(block_cid);
        let parent_receipts_jsonval : jsonrpsee::common::JsonValue = self.api.chain_get_parent_receipts(block_cid);
        
        let mut i : u32 = 0;
        let mut consumed_all_cid_msg_pairs = false;
        loop {
            let mut cid_str : String = "".to_string();
            let mut msg_builder = MessageBuilder::new();
            let mut msg_jsonval : &jsonrpsee::common::JsonValue = &jsonrpsee::common::JsonValue::Null;
            let receipt_jsonval : &jsonrpsee::common::JsonValue;

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

            // Did we cache msg_type_flag on a previous block?
            let mut msg_type_flag = MessageTypeFlag::Unknown;
            if self.incomplete_msg_cache.contains_key(&cid_str) {
                if let Some(mtf) = self.incomplete_msg_cache.remove(&cid_str) {
                    msg_type_flag = mtf;
                }
            }

            // Make the message struct
            let message : Message = msg_builder
                                .msg_fields(msg_jsonval)
                                .msg_type(msg_type_flag)
                                .receipt_field(receipt_jsonval)
                                .get();

            // Invoke callback. If return is true, save message struct
            each_complete_message(&cid_str, &message);

            i += 1;
        } // loop
    }

    // We iterate over the messasges in a block in order to add them to a list of messages we expect to 
    // see soon as parents (when they will have receipts).  For now we just store the msg_cid, the signature
    // type and the actual signature.
    //
    // Gets all the json back from api::chain_get_block_messages then
    // acts once one each {MsgCid,Message} pair found.
    pub fn iterate_over_all_messages_in_block(&mut self, block_cid: &str, 
        each_complete_message: std::option::Option<fn(msg_cid: &str,  msg: &Message)>,
        each_new_message:      std::option::Option<fn(msg_cid: &str)>)
    {
        // get block header and extract BLSAggregate from it
        let block_hdrs_jsonval : jsonrpsee::common::JsonValue = self.api.chain_get_block(block_cid);
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


        // Build vector of all cids in order
        let block_msgs_jsonval : jsonrpsee::common::JsonValue = self.api.chain_get_block_messages(block_cid);
        let mut msg_cid : String;
        let mut vd_msg_cids : std::collections::VecDeque<String> = 
            std::collections::VecDeque::new();
        let mut i : i32 = 0;
        loop {
            let json_path = format!("/Cids/{}/~1",i);
            if let Some(msg_cid_jsonval) = block_msgs_jsonval.pointer(&json_path) {
                msg_cid = msg_cid_jsonval.to_string();
                msg_cid = msg_cid[1..msg_cid.len()-1].to_string();
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
                    if let Some(f) = each_new_message {
                        f(&next_msg_cid);
                    }
                    self.incomplete_msg_cache.insert(next_msg_cid, msg_type);
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
                    if let Some(f) = each_new_message {
                        f(&next_msg_cid);
                    }
                    self.incomplete_msg_cache.insert(next_msg_cid, msg_type);
                } else {
                    assert!(false,"vd_msg_cids was empty prematurely: mismatch between number of message CIDs and number of BlsMessages+SecpkMessages");
                }
            } else {
                break;
            }
            i += 1;
        }

        //
        // Iterate the parents_messages and parents_receipts parts of this block (can skip if no callback)
        //
        if let Some(f) = each_complete_message {
            self.iterate_over_complete_messages_in_block(block_cid, f);
        }

        // assert that no msg_cids remain in queue
        assert_eq!(vd_msg_cids.len(),0,"All CIDs must now be exactly consumed");
    }
}


////////////////////////////////////////////////////////
/// 
/// Tipset
/// 
////////////////////////////////////////////////////////

pub struct Tipsets {
    i : usize,
    json_val : jsonrpsee::common::JsonValue,
}

impl Tipsets {
    pub fn new(api : &api::ApiClient, height: u64) -> Tipsets {
        Tipsets{
            i : 0,
            json_val: api.chain_get_tipset_by_height(height)
        }
    }
}

impl Iterator for Tipsets {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let json_path = format!("/Cids/{}/~1",self.i);
        self.i += 1;
        if let Some(jsonval) = self.json_val.pointer(&json_path) {
            if let Some(jsonval) = jsonval.as_str() {
                Some(jsonval.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
}


////////////////////////////////////////////////////////
/// 
/// ChainHeadBlocks
/// 
////////////////////////////////////////////////////////

pub struct ChainHeadBlock {
    pub height: u64,
    // TODO: add other fields
}

pub struct ChainHeadBlocks {
    i : usize,
    json_val : jsonrpsee::common::JsonValue,
}

impl ChainHeadBlocks {
    pub fn new(api : &api::ApiClient) -> ChainHeadBlocks {
        ChainHeadBlocks{
            i : 0,
            json_val: api.chain_head()
        }
    }
}

impl Iterator for ChainHeadBlocks {
    type Item = ChainHeadBlock;

    fn next(&mut self) -> Option<Self::Item> {
        let json_path = format!("/Blocks/{}/Height",self.i);
        self.i += 1;
        if let Some(jsonval) = self.json_val.pointer(&json_path) {
            if let Some(n) = jsonval.as_u64() {
                Some(ChainHeadBlock{
                    height: n,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}


////////////////////////////////////////////////////////
/// 
/// MaxTipsetHeight
/// 
////////////////////////////////////////////////////////

pub struct MaxTipsetHeight {
    pub max_height : u64,
}

impl MaxTipsetHeight {
    pub fn new(api : &api::ApiClient) -> MaxTipsetHeight {
        let max_height = ChainHeadBlocks::new(api)
            .map(|head_block| { head_block.height })
            .max()
            .unwrap_or(0u64);
        MaxTipsetHeight{
            max_height : max_height,
        }
    }
}

////////////////////////////////////////////////////////
/// 
/// iterate_over_blockchain() - main crate entrypoint
/// 
////////////////////////////////////////////////////////

pub fn iterate_over_blockchain(
    iterate_from_min_height:    u64,
    iterate_to_max_height:      u64,
    api:                        &crate::api::ApiClient, 
    on_starting_new_tipset:     std::option::Option<fn(height: u64, blocks: &Vec<String>)>,
    on_starting_block:          std::option::Option<fn(blk_cid: &str)>,
    on_found_new_message_cid:   std::option::Option<fn(msg_cid: &str)>,
    on_found_new_message:       std::option::Option<fn(msg_cid: &str, msg: &Message)>,
    on_finished_block:          std::option::Option<fn(blk_cid: &str)>,
    on_finished_tipset:         std::option::Option<fn(height: u64)>) 
{
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
    let mut i : u64 = max(iterate_from_min_height,0 as u64);
    log::info!("Iterating from height {} to {}",i,min(iterate_to_max_height,curr_tipset_height));
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
        if i > min(iterate_to_max_height,curr_tipset_height) {
            curr_tipset_height = MaxTipsetHeight::new(&api).max_height;
            if i > min(iterate_to_max_height,curr_tipset_height) {
                break
            }
        }
    }
}
