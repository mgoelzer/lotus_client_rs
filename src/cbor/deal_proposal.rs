use serde_cbor::value::Value;

#[derive(Debug)]
pub struct DealProposal {
    pub piece_cid: Vec<u8>,
    pub padded_piece_size: u64,
    pub is_verified_deal: bool,
    pub client_addr: Vec::<u8>,
    pub provider_addr: Vec::<u8>,
    pub label: String,
    pub start_epoch: u64,
    pub end_epoch: u64,
    pub storage_price_per_epoch: Vec::<u8>,
    pub provider_collateral: Vec::<u8>, 
    pub client_collateral: Vec::<u8>,
}

impl DealProposal {
    pub fn get_piece_cid_as_str(&self) -> String {
        let alpha = base32::Alphabet::RFC4648{padding:false};
        let mut s = base32::encode(alpha, &self.piece_cid);
        s = s.to_ascii_lowercase();
        println!("base32 of piece_cid='{}'",s);
        s
    }
}

pub fn decode_storage_deal(input_base64: &str) -> Option<DealProposal> {
    let byte_vec = base64::decode(input_base64).unwrap();

    let byte_slice: &[u8] = &byte_vec;
    let value : Value = match serde_cbor::from_slice(byte_slice) {
        Ok(val) => {
            val
        }, 
        Err(e) => {
            println!("decode_storage_deal: failed at serde_cbor::from_slice: {:?}",e);
            return None;
        }
    };
    //println!("{:?}", value);

    let mut piece_cid : Vec<u8>;
    let padded_piece_size : u64;
    let is_verified_deal: bool;
    let mut client_addr : Vec<u8>;
    let mut provider_addr : Vec<u8>;
    let label : String;
    let start_epoch : u64;
    let end_epoch : u64;
    let mut storage_price_per_epoch : Vec<u8>;
    let mut provider_collateral : Vec<u8>;
    let mut client_collateral : Vec<u8>;
    match value {
        Value::Array(vec) => {
            match &vec[0] {
                Value::Array(vec) => {
                    match &vec[0] {
                        Value::Array(vec) => {
                            match &vec[0] {
                                Value::Array(vec) => {
                                    match &vec[0] {
                                        Value::Bytes(bytes) => {
                                            piece_cid = vec![0; bytes.len()];
                                            piece_cid.copy_from_slice(&bytes);
                                        },
                                        _ => {
                                            println!("decode_storage_deal: failed at vec[0]");
                                            return None;
                                        }
                                    };
                                    match &vec[1] {
                                        Value::Integer(i) => {
                                            padded_piece_size = *i as u64;
                                        },
                                        _ => {
                                            println!("decode_storage_deal: failed at vec[1]");
                                            return None;
                                        }
                                    };
                                    match &vec[2] {
                                        Value::Bool(b) => {
                                            is_verified_deal = *b;
                                        },
                                        _ => {
                                            println!("decode_storage_deal: failed at vec[2]");
                                            return None;
                                        }
                                    };
                                    match &vec[3] {
                                        Value::Bytes(bytes) => {
                                            client_addr = vec![0; bytes.len()];
                                            client_addr.copy_from_slice(&bytes);
                                        },
                                        _ => {
                                            println!("decode_storage_deal: failed at vec[3]");
                                            return None;
                                        }
                                    };
                                    match &vec[4] {
                                        Value::Bytes(bytes) => {
                                            provider_addr = vec![0; bytes.len()];
                                            provider_addr.copy_from_slice(&bytes);
                                        },
                                        _ => {
                                            println!("decode_storage_deal: failed at vec[4]");
                                            return None;
                                        }
                                    };
                                    match &vec[5] {
                                        Value::Text(s) => {
                                            label = s.to_owned();
                                        },
                                        _ => {
                                            println!("decode_storage_deal: failed at vec[5]");
                                            return None;
                                        }
                                    };
                                    match &vec[6] {
                                        Value::Integer(i) => {
                                            start_epoch = *i as u64;
                                        },
                                        _ => {
                                            println!("decode_storage_deal: failed at vec[6]");
                                            return None;
                                        }
                                    };
                                    match &vec[7] {
                                        Value::Integer(i) => {
                                            end_epoch = *i as u64;
                                        },
                                        _ => {
                                            println!("decode_storage_deal: failed at vec[7]");
                                            return None;
                                        }
                                    };
                                    match &vec[8] {
                                        Value::Bytes(bytes) => {
                                            storage_price_per_epoch = vec![0; bytes.len()];
                                            storage_price_per_epoch.copy_from_slice(&bytes);
                                        },
                                        _ => {
                                            println!("decode_storage_deal: failed at vec[8]");
                                            return None;
                                        }
                                    };
                                    match &vec[9] {
                                        Value::Bytes(bytes) => {
                                            provider_collateral = vec![0; bytes.len()];
                                            provider_collateral.copy_from_slice(&bytes);
                                        },
                                        _ => {
                                            println!("decode_storage_deal: failed at vec[9]");
                                            return None;
                                        }
                                    };
                                    match &vec[10] {
                                        Value::Bytes(bytes) => {
                                            client_collateral = vec![0; bytes.len()];
                                            client_collateral.copy_from_slice(&bytes);
                                        },
                                        _ => {
                                            println!("decode_storage_deal: failed at vec[10]");
                                            return None;
                                        }
                                    };
                                },
                                _ => {
                                    println!("decode_storage_deal: failed at 4th inner array");
                                    return None;
                                },
                            }
                        },
                        _ => {
                            println!("decode_storage_deal: failed at 3rd inner array");
                            return None;
                        },
                    }
                },
                _ => {
                    println!("decode_storage_deal: failed at 2nd inner array");
                    return None;
                },
            }
        },
        _ => {
            println!("decode_storage_deal: failed at 1st inner array");
            return None;
        }
    }

    let dp = DealProposal{
        piece_cid: piece_cid.to_owned(),
        padded_piece_size: padded_piece_size,
        is_verified_deal: is_verified_deal,
        client_addr: client_addr,
        provider_addr: provider_addr,
        label: label,
        start_epoch: start_epoch,
        end_epoch: end_epoch,
        storage_price_per_epoch: storage_price_per_epoch,
        provider_collateral: provider_collateral,
        client_collateral: client_collateral,
    };

    //println!("dp={:?}",dp);
    Some(dp)
}
