#![allow(unused_variables)]
#![allow(dead_code)]

use serde_json::json;

static API_SERVER_IP_PORT : &str = "http://lotus1:1234/rpc/v0";

//////////////////////////////////////////////////////////////////////////////////////
//
// make_api_function macro to reduce boilerplate code identical in all api::* methods
//
//////////////////////////////////////////////////////////////////////////////////////

// TODO:  error handling instead of 3X unwraps...
macro_rules! make_api_function {
    ($method_call_name:literal, $auth_token_value:expr, $expr_evals_to_params:expr) => {
        async_std::task::block_on(async move {
            let transport = jsonrpsee::transport::http::HttpTransportClient::new(API_SERVER_IP_PORT, $auth_token_value);
            let mut raw_client = jsonrpsee::raw::RawClient::new(transport);
            let request_id = raw_client.start_request($method_call_name, $expr_evals_to_params).await.unwrap();
            jsonrpsee::common::from_value(
                raw_client.request_by_id(request_id).unwrap().await.unwrap() 
            )
            .unwrap() 
        })
    }
}

//
// Example of using make_api_function! macro:
//
// pub fn chain_get_tipset_by_height(height: u64) -> jsonrpsee::common::JsonValue {
//     make_api_function!("Filecoin.ChainGetTipSetByHeight","",{
//         let mut v_params : Vec<jsonrpsee::common::JsonValue> = vec!();
//         v_params.push(json!(height));
//         v_params.push(jsonrpsee::common::JsonValue::Array(vec!()));
//         let params = jsonrpsee::common::Params::Array(v_params);
//         params
//     })
// }
//


//////////////////////////////////////////////////////////////////////////////////////
//
// chain_get_tipset_by_height
//
//////////////////////////////////////////////////////////////////////////////////////

// equivalent to (for height=33):  `lotus chain list --height 33 --count 1` returning only the CIDs
// equivalentn curl (for height=33):  curl -X POST -H "Content-Type: application/json" 
//      --data '{ "jsonrpc": "2.0", "method": "Filecoin.ChainGetTipSetByHeight", 
//      "params":[33,[]], "id": 0 }' 'http://lotus1:1234/rpc/v0'
// (yes, the empty array is an essential parameter; it indicates 'types.EmptyTSK')
pub fn chain_get_tipset_by_height(height: u64) -> jsonrpsee::common::JsonValue {
    make_api_function!("Filecoin.ChainGetTipSetByHeight","",{
        let mut v_params : Vec<jsonrpsee::common::JsonValue> = vec!();
        v_params.push(json!(height));
        v_params.push(jsonrpsee::common::JsonValue::Array(vec!()));
        let params = jsonrpsee::common::Params::Array(v_params);
        params
    })
}


//////////////////////////////////////////////////////////////////////////////////////
//
// chain_head
//
//////////////////////////////////////////////////////////////////////////////////////

// equivalent to `lotus chain head`
// equivalentn curl:  curl -X POST -H "Content-Type: application/json" --data 
//      '{ "jsonrpc": "2.0", "method": "Filecoin.ChainHead", "params": null, "id": 0 }' 
//      'http://devnet1:1234/rpc/v0'
pub fn chain_head() -> jsonrpsee::common::JsonValue {
    make_api_function!("Filecoin.ChainHead","",{
        jsonrpsee::common::Params::None
    })
}


//////////////////////////////////////////////////////////////////////////////////////
//
// chain_get_block
//
//////////////////////////////////////////////////////////////////////////////////////


// equivalent curl:  curl -X POST -H "Content-Type: application/json"  \
//     --data '{ "jsonrpc": "2.0", "method": "Filecoin.ChainGetBlock", 
//     "params":[{"/":"bafy2bzacednghy3w2e42xqrgeisswleti2uf2dno4ffaieuge2wfrbigqwdv6"}], 
//     "id": 0 }' 'http://devnet1:1234/rpc/v0'
//
// where the returned json looks like this:
//
// {
//     "jsonrpc": "2.0",
//     "result": {
//       "Miner": "t01001",
//       "Ticket": {
//         "VRFProof": "pbrZ6m69mfzjLW0wYMgg7MQIPa1PZXZ0sGeUNwCzj0w7rJTy6T4L6hifD3EH2DilDqiVj7X8337WZsdv0VtSqyu2rIB3l2bP4H7CykO7ha5PnNkFHEzaKOzibScglYOQ"
//       },
//       "ElectionProof": {
//         "VRFProof": "uPLyTeVzSLcfH5T/4eb7H/tUV5IXPx0fNyRE7LHvQPWMZpD5a1JjAhLT6NIiaoPLDk0eHBD0uQzi9xADQ1lS4T1TgsfDkqytlTeB1owcsd/15KxBPN1fA7K0ZpbEMghP"
//       },
//       "BeaconEntries": [
//         {
//           "Round": 83217,
//           "Data": "tU0e3f/c6X4RuH7KfjRTQU1HvU7UeParVjqsE8ABQfy6kMPfZLorAd0duyTeq/EjEAuq1GkDJFcT/Adi0UOfWo9xC9ksl6B8AJFBN1S2fkxeLVNxF44AbbE4khkVSoKW"
//         }
//       ],
//       "WinPoStProof": [
//         {
//           "PoStProof": 3,
//           "ProofBytes": "qExIU5WZWjXMFO/vtrOH5aVgRnOj/RyOIfU5L0mE174hG6U8Nr7HkFvg9LLVFWcQq6kch4AbiQ9jM0dlm/MEgItMiarzZwe/JnyAkQEx5aET8bEeHFiQI+CRlT642ag2CId7IZOdQBttOoHZzD/EVdFwO+Sm7KzcfmFvSUrGuR1k6Jv8OTRsJkse/EuEQKrNlsTsZGYelQtxjnIEftEupxS1qdnJmdysL7vdCNrHMU65OKYuHVKbhn9MfSkg/Kvj"
//         }
//       ],
//       "Parents": [
//         {
//           "/": "bafy2bzacecwan7xlr3lwroihugoanxctki2tmrrb6inxeqwkmwcyqwo7rcn3e"
//         },
//         {
//           "/": "bafy2bzacecncbsprbp2jy2olzingzqyeolehavjl7h7qkfzmplawucqcklsxy"
//         },
//         {
//           "/": "bafy2bzaceah5chysygz3uqjfcvk2ynnwzmjbdt3rbcyoo7zaveipxegbpoe6q"
//         }
//       ],
//       "ParentWeight": "475776",
//       "Height": 33,
//       "ParentStateRoot": {
//         "/": "bafy2bzacedggyxat7lp72fsazi7nmpl3longwd6tr7pbgib7gfrwzichrjvyy"
//       },
//       "ParentMessageReceipts": {
//         "/": "bafy2bzacecxy5u5ecfp2czydqpryksofscrai4o2vd6zvwevtl7zwtqzxa6ca"
//       },
//       "Messages": {
//         "/": "bafy2bzacecuzfokl72hhpqnuucb6f3prqapwsppitgko5aiuremjps6gdo5zy"
//       },
//       "BLSAggregate": {
//         "Type": 2,
//         "Data": "ks13QIzI2lc3S9EXnkQvAdR5pHqU6vGgzoIsClm4Ilx9FasofpgNheJlJ7L4NRhIFPOgm9xtqOe4ruzw9RusGLctIMJ0bImV+GNuGaI51o3ZYxfaRK219qF4I6L9suok"
//       },
//       "Timestamp": 1592525625,
//       "BlockSig": {
//         "Type": 2,
//         "Data": "ga6uferE0fFX2/5ngENENudGlFqKE3l8KMzTdZ18bsOB1E0yKXzcHQzbztxVzvbEFhV/rvBQ0jpsiHAta9asHMPM6oOF1T4Pz4KN7/23Na3UNOICQrkDTF2+615cASAG"
//       },
//       "ForkSignaling": 0
//     },
//     "id": 0
//   }
//
// Notes:
// - These are just the block headers and don't contain any of the cids or cid data.
pub fn chain_get_block(block_cid: &str) -> jsonrpsee::common::JsonValue {
    make_api_function!("Filecoin.ChainGetBlock","",{
        let mut v_params : Vec<jsonrpsee::common::JsonValue> = vec!();
        v_params.push(json!({"/":block_cid}));
        let params = jsonrpsee::common::Params::Array(v_params);
        params
    })
}


//////////////////////////////////////////////////////////////////////////////////////
//
// chain_get_block_messages
//
//////////////////////////////////////////////////////////////////////////////////////

// equivalent curl:  curl -X POST -H "Content-Type: application/json" 
//      --data '{ "jsonrpc": "2.0", "method": "Filecoin.ChainGetBlockMessages", 
//      "params":[{"/":"bafy2bzacebtfstfnqx7sc6phjxbdzin6vm4gvel5vcnnoqm6eegum3cj5bdra"}], 
//      "id": 0 }' 'http:/lotus1:1234/rpc/v0'
//
// which returns this structure:
//
// {
//     "jsonrpc": "2.0",
//     "result": {
//       "BlsMessages": [
//         {
//           "Version": 0,
//           "To": "t01002",
//           "From": "t3wowfuawcs6dwddo75jum2ddccrrq376oildvblukm4ctybuo5wtylvht5rahhtsoohoca3vdmyyt2zjlwqoa",
//           "Nonce": 0,
//           "Value": "0",
//           "GasPrice": "0",
//           "GasLimit": 99999999,
//           "Method": 4,
//           "Params": "gVgmACQIARIgxk09Mw1Kika9rMn0cUU05TvWiz3FmsJSiTqjTVoO34E="
//         },
//       ],
//       "SecpkMessages": [
//         {
//            "Message": {
//              "Version": 0,
//              "To": "t3u2ubtrhyfpky53hq3k4l3xm7e74vzsrr7lmrx6fph6oesz4u7hy6nd2sosj6x2yu4zgbvhbkawvc22oblzta",
//              "From": "t1hw4amnow4gsgk2ottjdpdverfwhaznyrslsmoni",
//              "Nonce": 1565,
//              "Value": "50000000000000000000",
//              "GasPrice": "0",
//              "GasLimit": 10000,
//              "Method": 0,
//              "Params": ""
//            },
//            "Signature": {
//              "Type": 1,
//              "Data": "8LHFp+nM5ozmQhnESqXRilt0+molZpT+ytXqeVcYB+4rEEA7hFgFnMhF8WEbfhPiT4uisZW/7Nr1wVo590fJXQE="
//            }
//         }
//       ],
//       "Cids": [
//         {
//           "/": "bafy2bzaceahhze5r6ceij7ctcxadje2miweqxzcuuamzup2k5bb423tyoopve"
//         },
//         {
//           "/": "bafy2bzaced6u2e2s7uyy44lhavoexym7oyzlqnowa36ympogpfmqnzl5l2d36"
//         }
//       ]
//     },
//     "id": 0
//   }
//
// where:
// - The order of the CIDs is always this:  1st BLS message, 2nd BLS message, ..., last BLS message, 
// 1st Secp message, 2nd secp message, ..., last secp message
// 
pub fn chain_get_block_messages(block_cid: &str) -> jsonrpsee::common::JsonValue {
    make_api_function!("Filecoin.ChainGetBlockMessages","",{
        let mut v_params : Vec<jsonrpsee::common::JsonValue> = vec!();
        v_params.push(json!({"/":block_cid}));
        let params = jsonrpsee::common::Params::Array(v_params);
        params
    })
}


//////////////////////////////////////////////////////////////////////////////////////
//
// chain_get_parent_messages
//
//////////////////////////////////////////////////////////////////////////////////////

// equivalent curl:  curl -X POST -H "Content-Type: application/json" --data 
//      '{ "jsonrpc": "2.0", "method": "Filecoin.ChainGetParentMessages", 
//      "params":[{"/":"bafy2bzacebc65yakb26o5f5jsscn2mkhxvkafocqho2wghg7m7hqlntk5iwwe"}], 
//      "id": 0 }' 'http:/lotus1:1234/rpc/v0'
//
// produces this output:
//
// {
//     "jsonrpc": "2.0",
//     "result": [
//       {
//         "Cid": {
//           "/": "bafy2bzacebwydbfh32tilfdokxdm7u5vubvhoadhn6ieafd7dyh6s3q52h33i"
//         },
//         "Message": {
//           "Version": 0,
//           "To": "t04",
//           "From": "t3rglqgjsidwb3l3j3uz52wylctnec2l5b4dtmoeebwslcktxzcvf7sdztqknhtxoyzizcuvetsdpa5cj743ea",
//           "Nonce": 20,
//           "Value": "0",
//           "GasPrice": "0",
//           "GasLimit": 10000000,
//           "Method": 2,
//           "Params": "hVgxA4mXAyZIHYO17Tume6thYptILS+h4ObHEIG0liVO+RVL+Q8zgpp53djKMipUk5DeDlgxA4mXAyZIHYO17Tume6thYptILS+h4ObHEIG0liVO+RVL+Q8zgpp53djKMipUk5DeDgNYJgAkCAESIOJIJbNOiVvz4WeKofs97SLd1uAgCHKKQ12uPu+r1qcngA=="
//         }
//       },
//     ]
// }
//
// where:
// - There can be any number of Cid,Message pair blocks like above.
pub fn chain_get_parent_messages(block_cid: &str) -> jsonrpsee::common::JsonValue {
    make_api_function!("Filecoin.ChainGetParentMessages","",{
        let mut v_params : Vec<jsonrpsee::common::JsonValue> = vec!();
        v_params.push(json!({"/":block_cid}));
        let params = jsonrpsee::common::Params::Array(v_params);
        params
    })
}


//////////////////////////////////////////////////////////////////////////////////////
//
// chain_get_parent_receipts
//
//////////////////////////////////////////////////////////////////////////////////////

// equivalent curl:  curl -X POST -H "Content-Type: application/json" --data 
//      '{ "jsonrpc": "2.0", "method": "Filecoin.ChainGetParentReceipts", 
//      "params":[{"/":"bafy2bzacebc65yakb26o5f5jsscn2mkhxvkafocqho2wghg7m7hqlntk5iwwe"}], 
//      "id": 0 }' 'http:/lotus1:1234/rpc/v0' | jq
//
// which produces this json:
//
//   {
//     "jsonrpc": "2.0",
//     "result": [
//       {
//         "ExitCode": 0,
//         "Return": "gkMA0yhVApbyVpWf0hHUCUM+VYpeHKIODLXL",
//         "GasUsed": 33928
//       },
//       {
//         "ExitCode": 0,
//         "Return": "gkMA1ChVAiU2nVDS/oE2R/lxpgsGwIGHcMcq",
//         "GasUsed": 33483
//       },
//     ],
//     "id": 0
//   }
//
// where
// - The order of the receipts here corresponds to the order of the Cid,Message pairs in 
// the response to Filecoin.ChainGetParentMessages.
//
pub fn chain_get_parent_receipts(block_cid: &str) -> jsonrpsee::common::JsonValue {
    make_api_function!("Filecoin.ChainGetParentReceipts","",{
        let mut v_params : Vec<jsonrpsee::common::JsonValue> = vec!();
        v_params.push(json!({"/":block_cid}));
        let params = jsonrpsee::common::Params::Array(v_params);
        params
    })
}


////////////////////////////////////////////////////////////////////
///
/// Tests
/// 
////////////////////////////////////////////////////////////////////

#[cfg(test)]
#[allow(unused_imports)]
mod tests { 
    use super::*;
    use std::panic;

    // TODO:  once errors are being handled properly instead of unwraps, add
    // tests that check error conditions and get rid of all the catch_unwind()

    // Mainly just test for no unwrap panics, and some output returned

    #[test]
    fn test_chain_get_tipset_by_height() {
        let res = panic::catch_unwind(|| {
            let jsonval : jsonrpsee::common::JsonValue = super::chain_get_tipset_by_height(0);
            assert_ne!(jsonval.to_string(),"","Result contains nonempty json");
            0
        });
        assert!(res.is_ok());
    }

    #[test]
    fn test_chain_head() {
        let res = panic::catch_unwind(|| {
            let jsonval : jsonrpsee::common::JsonValue = super::chain_head();
            assert_ne!(jsonval.to_string(),"","Result contains nonempty json");
            0
        });
        assert!(res.is_ok());
    }


    #[test]
    fn test_chain_get_block() {
        let mut jsonval = chain_get_tipset_by_height(1);
        let mut cid = jsonval.pointer("/Cids/0/~1").unwrap().to_string();
        cid = cid[1..cid.len()-1].to_string();
        //println!(">>> cid='{}'",cid);

        let res = panic::catch_unwind(|| {
            let jsonval : jsonrpsee::common::JsonValue = super::chain_get_block(&cid);
            assert_ne!(jsonval.to_string(),"","Result contains nonempty json");
            0
        });
        assert!(res.is_ok());
    }
}
