#![allow(unused_variables)]
#![allow(dead_code)]

use serde_json::json;
use log;

//////////////////////////////////////////////////////////////////////////////////////
//
// make_api_function macro - used to reduce boilerplate code im api methods
//
//////////////////////////////////////////////////////////////////////////////////////

// TODO:  error handling instead of 3X unwraps...
macro_rules! make_api_function {
    ($selfparam:ident, $method_call_name:literal, $auth_token_value:expr, $expr_evals_to_params:expr) => {
        async_std::task::block_on(async move {
            let transport = jsonrpsee::transport::http::HttpTransportClient::new(&$selfparam.endpoint_url, $auth_token_value);
            let mut raw_client = jsonrpsee::raw::RawClient::new(transport);
            match raw_client.start_request($method_call_name, $expr_evals_to_params).await {
                Ok(request_id) => {
                    match raw_client.request_by_id(request_id) {
                        Some(fut) => {
                            match fut.await {
                                Ok(jsonval) => {
                                    match jsonrpsee::common::from_value::<jsonrpsee::common::JsonValue>(jsonval) {
                                        Ok(ret) => {
                                            ret
                                        },
                                        Err(e) => {
                                            jsonrpsee::common::JsonValue::Null
                                        }
                                    }
                                },
                                Err(e) => {
                                    jsonrpsee::common::JsonValue::Null
                                }
                            }
                        },
                        None => {
                            jsonrpsee::common::JsonValue::Null
                        }
                    }
                },
                Err(e) => {
                    jsonrpsee::common::JsonValue::Null
                }
            }
        })
    }
}

//
// Example of using make_api_function! macro to make an api calling function:
//
// pub fn chain_get_tipset_by_height(height: u64) -> jsonrpsee::common::JsonValue {
//     make_api_function!("Filecoin.ChainGetTipSetByHeight",                # Specify API method name
//                        "",                                               # Optionally provide auth token
//     {            
//         let mut v_params : Vec<jsonrpsee::common::JsonValue> = vec!();   # In this block, build
//         v_params.push(json!(height));                                    # an object that mimics the
//         v_params.push(jsonrpsee::common::JsonValue::Array(vec!()));      # the JSON RPC field 'params'
//         let params = jsonrpsee::common::Params::Array(v_params);         # for this call. Return it from
//         params                                                           # the block.
//     })
// }
//

enum AuthToken {
    None,
    Value(String),
}

pub struct ApiClient {
    endpoint_url: String,
    auth_token: AuthToken,
}

impl ApiClient {
    pub fn new(endpoint_url: &str) -> ApiClient {
        ApiClient{
            endpoint_url : endpoint_url.to_string(),
            auth_token : AuthToken::None,
        }
    }

    pub fn auth_token(& mut self, auth_token: &str) {
        assert_ne!(auth_token,"");
        self.auth_token = AuthToken::Value(String::from(auth_token));
    }

    pub fn endpoint(& mut self, endpoint: &str) {
        self.endpoint_url = endpoint.to_string();
    }

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
    pub fn chain_get_tipset_by_height(&self, height: u64) -> jsonrpsee::common::JsonValue {
        make_api_function!(self, "Filecoin.ChainGetTipSetByHeight","",{
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

    // Equivalent to `lotus chain head`
    // Equivalentn curl:  curl -X POST -H "Content-Type: application/json" --data 
    //      '{ "jsonrpc": "2.0", "method": "Filecoin.ChainHead", "params": null, "id": 0 }' 
    //      'http://devnet1:1234/rpc/v0'
    // 
    // where the returned json looks like:
    //
    // {
    //     "jsonrpc": "2.0",
    //     "result": {
    //       "Cids": [
    //         {
    //           "/": "bafy2bzacebazmwzz7v5svaqwxhuk26jtco526k2vlzb5dt4ii6cuoszmq5hx4"
    //         },
    //         {
    //           "/": "bafy2bzacedy4xlejebb5gbxx2eqv5cnexywbshxbqvh7cmssd6rhw7znxupzy"
    //         }
    //       ],
    //       "Blocks": [
    //         {
    //           "Miner": "t032055",
    //           "Ticket": {
    //             "VRFProof": "g1677uluuxPdmTZq/B0MS/rhLoH/Pr1OtEhAuVCX1Ds6YnYYCb2qn5YcKyZPB2KkFEYU0QmkAfp+GuksaU6oLjsBqxMIKbsUj0qBHPA5GwaLRLPlUPs+W4OaVW1cKXfW"
    //           },
    //           "ElectionProof": {
    //             "VRFProof": "o+B9FEz5gtu3XQY5BUJCaARINIw1tGCQ04y1+YY16CwmxqJ3Mb5tdEKepCPtBDINBg4Y4Gt8c+QJau+ufJWQL25T+pzqg44WyB7XHU14tk03Q+vVuN1VLO33+xW12O/v"
    //           },
    //           "BeaconEntries": [
    //             {
    //               "Round": 180406,
    //               "Data": "mDE3PLUkCsu9guRKY5QmXPhSuR8OGmZY8BmkOZSsgu5Y0IghOY/HagN8p0XxWuJAGT3nkyhdDGX5uzKm9X+8PqonLafchGw4dRwGQ9VBALVzfXTEs92fvo1U8jDriH0s"
    //             }
    //           ],
    //           "WinPoStProof": [
    //             {
    //               "PoStProof": 3,
    //               "ProofBytes": "gl3lowgpjwZgyyTFhgeKos5O9gmehe+g5zYfruZxGTE48MatEal1vwslkNGbctLKmO0mggs+ZQZXWySa8L8qrRuqWxZ+6ovibWJqR8NTQ83WE0djj6cshiHApD7jEm5CGZtzUtT/zPWROFbHNFYumso5jf/vvx4hULg5yBRDSCKoVBqKH9dVFuuN6SCGbFGFsX4TP+TLqvemXnaDgzIyZWzFwz6SruIf04o7nCP5ifCrf+4rdKBnxpRn8WBQz+A8"
    //             }
    //           ],
    //           "Parents": [
    //             {
    //               "/": "bafy2bzacedxg6sc42mcrwhlzvgjqvnxzgznwthp5yen2suyslwpdggf2fxou2"
    //             },
    //             [..deleted...]
    //           ],
    //           "ParentWeight": "1689488189",
    //           "Height": 97222,
    //           "ParentStateRoot": {
    //             "/": "bafy2bzacebmjx4ipxmldy3zbnitstjcgxibyibbs64mvwvek6czxhskgawhmk"
    //           },
    //           "ParentMessageReceipts": {
    //             "/": "bafy2bzacebeapgu2cyzt5i4wfm44jior4u2akgwdzv7ko2hayslkzozeyl3pw"
    //           },
    //           "Messages": {
    //             "/": "bafy2bzaceb3xfzsg2xl4v5yfdecrqlpno4nly7j42zrkppozyp74fv7rlj7hc"
    //           },
    //           "BLSAggregate": {
    //             "Type": 2,
    //             "Data": "idZteeT5Nz2KHIZf8+ibniX72tO7EBzJ/xd84y6a+gXCnBV++P4CEdj83FMQtdjIFPNB5OHJgRKo0AmFcoGCPnEtDEkc2kbTmgQn4fPjXp5yjTn0cPROYB98abNEHUgV"
    //           },
    //           "Timestamp": 1594955350,
    //           "BlockSig": {
    //             "Type": 2,
    //             "Data": "hu7rQakL9Mn8xW11rZVPPm/zuZyOnCpgpahthO1iPB1JpUYPenbBoinIkOgZXzfKF9yP+y9MffHj88QYCMzAxsrBCwryIbDaEcHlxbZHOBXEeI/29rssqqGND7eIDiPf"
    //           },
    //           "ForkSignaling": 0
    //         },
    //         [...deleted (more miners)...]
    //       ],
    //       "Height": 97222
    //     },
    //     "id": 0
    //   }
    pub fn chain_head(&self) -> jsonrpsee::common::JsonValue {
        make_api_function!(self, "Filecoin.ChainHead","",{
            jsonrpsee::common::Params::None
        })
    }

    //////////////////////////////////////////////////////////////////////////////////////
    //
    // check_endpoint_connection
    //
    //////////////////////////////////////////////////////////////////////////////////////

    // Tries to make a simple API query to test the connection.
    //
    // Returns:  true if the test query was successful, otherwise false.
    pub fn check_endpoint_connection(&self) -> bool {
        log::debug!("check_endpoint_connection: for endpoint '{}'",self.endpoint_url);

        let _height : u64;

        let ret_jsonval : jsonrpsee::common::JsonValue;
        ret_jsonval = self.chain_head();
        let ret_jsonval_option = ret_jsonval.pointer("/Height");
        let height_jsonval : jsonrpsee::common::JsonValue;
        if let Some(height_jsonval) = &ret_jsonval_option {
            log::info!("check_endpoint_connection: got height_jsonval='{}'",height_jsonval.to_string());

            let height_option : Option<u64> = height_jsonval.as_u64();
            if let Some(number) = height_option {
                _height = number;
                log::info!("check_endpoint_connection: got height='{}'",_height);
            } else {
                log::debug!("check_endpoint_connection: aborted at u64 unwrap");
                return false;
            }
        } else {
            log::debug!("check_endpoint_connection: aborted at Height unwrap");
            return false;
        }
        log::debug!("check_endpoint_connection: endpoint OK '{}'",self.endpoint_url);
        true
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
    //
    pub fn chain_get_block(&self, block_cid: &str) -> jsonrpsee::common::JsonValue {
        make_api_function!(self, "Filecoin.ChainGetBlock","",{
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
    pub fn chain_get_block_messages(&self, block_cid: &str) -> jsonrpsee::common::JsonValue {
        make_api_function!(self, "Filecoin.ChainGetBlockMessages","",{
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
    //
    pub fn chain_get_parent_messages(&self, block_cid: &str) -> jsonrpsee::common::JsonValue {
        make_api_function!(self, "Filecoin.ChainGetParentMessages","",{
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

    // Equivalent curl:  curl -X POST -H "Content-Type: application/json" --data 
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
    pub fn chain_get_parent_receipts(&self, block_cid: &str) -> jsonrpsee::common::JsonValue {
        make_api_function!(self, "Filecoin.ChainGetParentReceipts","",{
            let mut v_params : Vec<jsonrpsee::common::JsonValue> = vec!();
            v_params.push(json!({"/":block_cid}));
            let params = jsonrpsee::common::Params::Array(v_params);
            params
        })
    }
}