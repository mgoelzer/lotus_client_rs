## lotus_chain\_rs:  A Rust Crate for Reading the Filecoin Blockchain

`lotus_chain_rs` is a rust crate that provides structures and methods for iterating over the Filecoin blockchain.  It uses a callback interface to dependency inject your code as it discovers each message within each block.

- [Examples](#examples)
- [Compiling and Running](#compiling-and-running)
- [Prerequisite Configuration of Your Lotus Node](#prerequisite-configuration-of-your-lotus-node)
- [Contributing](#contributing)

### Examples

For example, to print every message as we walk the blocks of the chain:

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
        println!("\n--- message {} ---\n{}{}", msg_cid, msg, "-".to_string().repeat(80));
    };

    // Run iterate_over_blockchain with our callbacks on the first few blocks
    iterate_over_blockchain(0, 5, &api, 
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

Here's another example.  It searches the chain for all messages to or from a wallet starting with prefix `t3`:

```
use lotus_client_rs::blockanalyzer::{Message,iterate_over_blockchain};

fn main() {
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
```

which might produce this output:

```
Tipset height: 0
Tipset height: 1
Tipset height: 2
Message bafy2bzacealrnqdpalpujesp3yck2o2o2fynak27opsxxpdw43zap746bcmm2:
  From t3wvo6nwqjw6pbrxryqmo2a2shw5jyobyg5qtaehan66hvl5jojs76m7av3si566clavyscyydvpcrs4fryqmq
  To t01000

Message bafy2bzaceayyvchzs34byt72pzgpcnuea7pfpei34i2io32oqyc7duat27u6m:
  From t3tewri2g72t33gq2znkmqsozk6gdbjw7bsjrkqrmbjm6grzzyvnw3nhwvdflepp26utsro246gpue7ht3kwda
  To t01002

Tipset height: 3
Tipset height: 4
Message bafy2bzacebkegb6az5n7yhjxtveayhtlgut733eg5sutujbj2atmfiaufvemk:
  From t3sfwg75akvhkliomoj6kpvgrxucjk66b57knwtcf3bd5qotl3lbolwdvwsorwwjnuawfsrvctrz3o3i6snx5a
  To t01001

Message bafy2bzacecdxtztvt3thlgihmj5okbfnk73gl4qlxigr6q77n7t2scfcrtans:
  From t12cho7aqks2aon7ytfs3kk5whxwjabxfi2cdtx3i
  To t3vc6iop22qpigsm7g2pjhfuu6dhs4liiblx3gbc7y6gkbblwoumqi6jtqfza3eorrwdxgo25f7daumoycpvva

Tipset height: 5
Tipset height: 6
Tipset height: 7
Tipset height: 8
[...et cetera...]
```

(See [examples/find-by-wallet.rs](examples/find-by-wallet.rs))

## `cid_oracle`

The crate [`cid_oracle`](https://github.com/mgoelzer/cid_oracle/) depends on `lotus_client_rs` to mines the chain for specific types of messages, caching them as they're found, and serving them on an HTTP API.  [Read more](https://github.com/mgoelzer/cid_oracle/blob/master/README.md).

## Compiling and Running

To run the examples:

```
cargo build --examples
./target/debug/examples/print-everything
./target/debug/examples/find-by-wallet
```

To use this crate in your own project, add to your Cargo.toml:

```
lotus_client_rs = { git = "https://github.com/mgoelzer/lotus_client_rs", branch = "master" }
```

## Prerequisite Configuration of Your Lotus Node

For the above examples to actually work as advertised, you'll need to repalce `http://lotus1:1234/rpc/v0` with the address of your own Lotus instance.  Make sure you set up your Lotus instance just right:

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

- Make sure port 1234 is open on your node's firewall.  (For example, `sudo ufw allow 1234`.)


## Contributing

Contributions are welcome!  Start by checking the [issues](/issues), or propose a better way to implement this crate.