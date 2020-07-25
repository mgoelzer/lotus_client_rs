## Crate lotus_chain\_rs

`lotus_chain_rs` is a rust crate that provides structures and methods for iterating over the Filecoin blockchain.  It uses a
callback interface to dependency inject your code as it discovers each block and each message within.

For example, to print every message grouped by block:

```rust
use lotus_client_rs::blockanalyzer::{Message,iterate_over_blockchain};

fn main() {
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
        println!("\n--- message {} ---\n{}--------------------------------------------------------------------------------",msg_cid,msg);
    };

    // Run iterate_over_blockchain with our callbacks on the first few blocks
    iterate_over_blockchain(0, 3, &api, 
        Some(on_start_new_tipset),
        Some(on_start_new_block),
        None,
        Some(on_found_new_message),
        None,
        None
    );
}
```

which will produce results like:

```
% ./target/debug/examples/print-everything
Height 0
  Block: bafy2bzacebskopykivrgmlayzdd2lnsnzxhlpomx3gr3ldmo43imk2fimxqts
Height 1
  Block: bafy2bzacebskopykivrgmlayzdd2lnsnzxhlpomx3gr3ldmo43imk2fimxqts
Height 2
  Block: bafy2bzacebsgavbckogjotgewovo7oyq66y3xfbi2fffvyrqkzb3juejyilvo
Height 3
  Block: bafy2bzacebs5tnnzosyngzaffypk3wxyvt6xtafkhy2bnxxid2zabajxyjqxk

--- message bafy2bzacecveqyivxeo33auajdh5w3kgyhc2qfahzo5ayx53aqce4spmz6lum ---
To: t01002
From: t3wzcpwznw6dvl6x3beekspluvhdwh26h3tvmw5y2fychse7pr6xlsfmxuhsv6ki7r3pm6s7gxc65h52lgqfsa
Type: BlsMessage(BlsAggregateSignature { type_num: 2, data: "kv+d88i5ehGDE4MpFl8pSI2aLGu2MLnIfwZpX5pttL7yqogGoRvQLW7BTEPuaYsxGObiKKQjJ2EZmmkvrTC24ziFy7DmZ3EIlcQ9jCFM+OBolM54dsPaE3d1xOcYZ5Gv" })
Version: 0
Nonce: 0
Value: 0
Gas price: 1
Gas limit: 2725107
Method: 4
Params: gVgmACQIARIgPZaTliZwVx99094NbOU9Tda8RtfpAROjYrtGBB9kJE8=
Receipt: Receipt(ReceiptFields { exit_code: 0, ret: "\"\"", gas_used: 2314923 })
--------------------------------------------------------------------------------

--- message bafy2bzacebkp3235rv6cg53eyy2nqrmeva4jb73fsmvf3hzrjplcwr5ux45ng ---
To: t01001
From: t3vzz5pjcoaessfgq3qsw3c2xjsynqyenqydhu5bvh4qodp4xkqg6jljrvtlnjrb3uxvzghgdmjrxj4gndyjva
Type: BlsMessage(BlsAggregateSignature { type_num: 2, data: "kv+d88i5ehGDE4MpFl8pSI2aLGu2MLnIfwZpX5pttL7yqogGoRvQLW7BTEPuaYsxGObiKKQjJ2EZmmkvrTC24ziFy7DmZ3EIlcQ9jCFM+OBolM54dsPaE3d1xOcYZ5Gv" })
Version: 0
Nonce: 0
Value: 0
Gas price: 1
Gas limit: 2726307
Method: 4
Params: gVgmACQIARIg15C2IbSTsQrSBazRkiLx7Vraidtx/FPr8sMHi/SI2iM=
Receipt: Receipt(ReceiptFields { exit_code: 0, ret: "\"\"", gas_used: 2315923 })
--------------------------------------------------------------------------------

--- message bafy2bzacedmgp3qdtlrzuq54j4h4kjassnwl6vp7rdw3anuqffd72smqi7hyq ---
To: t01000
From: t3q5ipevdz43hlwcdy7abyufknrt36fym324nkie7llkguqblwg4db6jydvkdz4sj5ixibzlwjcyjzuntus6dq
Type: BlsMessage(BlsAggregateSignature { type_num: 2, data: "kv+d88i5ehGDE4MpFl8pSI2aLGu2MLnIfwZpX5pttL7yqogGoRvQLW7BTEPuaYsxGObiKKQjJ2EZmmkvrTC24ziFy7DmZ3EIlcQ9jCFM+OBolM54dsPaE3d1xOcYZ5Gv" })
Version: 0
Nonce: 0
Value: 0
Gas price: 1
Gas limit: 2726307
Method: 4
Params: gVgmACQIARIggbS+Zl7gToe6lSxqjeekA33TmxJPAD8eoySeHobE6gE=
Receipt: Receipt(ReceiptFields { exit_code: 0, ret: "\"\"", gas_used: 2315923 })
--------------------------------------------------------------------------------
```

(See [examples/print-everything.rs](examples/print-everything.rs))

This example searches the chain for all messages to or from the wallet address `t3wzcpwznw6dvl6x3beekspluvhdwh26h3tvmw5y2fychse7pr6xlsfmxuhsv6ki7r3pm6s7gxc65h52lgqfsa`:

```
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
```

which might produce this output:

```
(at height 0-9)
Message bafy2bzacecveqyivxeo33auajdh5w3kgyhc2qfahzo5ayx53aqce4spmz6lum:
  From t3wzcpwznw6dvl6x3beekspluvhdwh26h3tvmw5y2fychse7pr6xlsfmxuhsv6ki7r3pm6s7gxc65h52lgqfsa
  To t01002

Message bafy2bzacecveqyivxeo33auajdh5w3kgyhc2qfahzo5ayx53aqce4spmz6lum:
  From t3wzcpwznw6dvl6x3beekspluvhdwh26h3tvmw5y2fychse7pr6xlsfmxuhsv6ki7r3pm6s7gxc65h52lgqfsa
  To t01002

(at height 10-19)
(at height 20-29)
(at height 30-39)
(at height 40-49)
```

(See [examples/find-by-wallet.rs](examples/find-by-wallet.rs))

## Crate 'indexer'

The crate [`indexer`](indexer/) in this repo is another example that uses `lotus_client_rs`.  `indexer` mines the chain for specific types of messages, caches them, and serves them on an HTTP API.

## Configuring a Lotus Machine

To run the above examples, you'll need to repalce `http://lotus1:1234/rpc/v0` with the address of your own Lotus instance.  Set up a Lotus instance like this:

- Install Lotus (instructions for [Arch](https://lotu.sh/en+install-lotus-arch), [Ubuntu](https://lotu.sh/en+install-lotus-ubuntu), [Fedora](https://lotu.sh/en+install-lotus-fedora), [macOS](https://lotu.sh/en+install-lotus-macos))
- Make sure your Lotus is fully synced: `lotus sync wait` (may take days)
- Edit `.lotus/config.toml` to enable the JSON RPC API:

	```
	[API]
	ListenAddress = "/ip4/0.0.0.0/tcp/1234/http"
	RemoteListenAddress = "/ip4/0.0.0.0/tcp/1234/http"
	```
	
- Restart the Lotus daemon to pick up the config changes:

	```
	killall lotus
	nohup lotus daemon > ~/lotus-daemon.log &
	```

## Compiling and Running

To compile, run and use the code in this repo:

```
git clone https://github.com/mgoelzer/filecoin-chain-rs
cd filecoin-chain-rs

# Running the examples
cargo build --examples
./target/debug/examples/print-everything
./target/debug/examples/find-by-wallet

# Running indexer
cargo run -p indexer -- --endpoint="http://lotus1:1234/rpc/v0"

# To use in your own project, make your project directory a
# sibling of filecoin-chain-rs; then put in your Cargo.toml:
lotus_client_rs = { path = "../lotus_client_rs" }
```