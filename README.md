## Crate lotus_chain\_rs

`lotus_chain_rs` is a rust crate that provides structures and methods for iterating over the Filecoin blockchain.  It uses a
callback interface to dependency inject your code as it discovers each block and each message within.

For example, to print every message CID grouped by block:

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
        println!("    Block: {}",blkcid);
    };
    let on_found_new_message = |msg_cid:&str, _msg:&Message| {
        println!("        Msg Cid: {}",msg_cid);
    };

    // Run iterate_over_blockchain with our callbacks on the first 5 blocks
    iterate_over_blockchain(0, 4, &api, 
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
Height 0
    Block: bafy2bzacebskopykivrgmlayzdd2lnsnzxhlpomx3gr3ldmo43imk2fimxqts
Height 1
    Block: bafy2bzacebskopykivrgmlayzdd2lnsnzxhlpomx3gr3ldmo43imk2fimxqts
Height 2
    Block: bafy2bzacebsgavbckogjotgewovo7oyq66y3xfbi2fffvyrqkzb3juejyilvo
Height 3
    Block: bafy2bzacebs5tnnzosyngzaffypk3wxyvt6xtafkhy2bnxxid2zabajxyjqxk
        Msg Cid: bafy2bzacecveqyivxeo33auajdh5w3kgyhc2qfahzo5ayx53aqce4spmz6lum
        Msg Cid: bafy2bzacebkp3235rv6cg53eyy2nqrmeva4jb73fsmvf3hzrjplcwr5ux45ng
        Msg Cid: bafy2bzacedmgp3qdtlrzuq54j4h4kjassnwl6vp7rdw3anuqffd72smqi7hyq
Height 4
    Block: bafy2bzacebs5tnnzosyngzaffypk3wxyvt6xtafkhy2bnxxid2zabajxyjqxk
        Msg Cid: bafy2bzacecveqyivxeo33auajdh5w3kgyhc2qfahzo5ayx53aqce4spmz6lum
        Msg Cid: bafy2bzacebkp3235rv6cg53eyy2nqrmeva4jb73fsmvf3hzrjplcwr5ux45ng
        Msg Cid: bafy2bzacedmgp3qdtlrzuq54j4h4kjassnwl6vp7rdw3anuqffd72smqi7hyq
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
            println!("(at height {})",height);
        }
    };
    let on_msg = |msg_cid:&str, msg:&Message| {
        let find_addr = "t3wzcpwznw6dvl6x3beekspluvhdwh26h3tvmw5y2fychse7pr6xlsfmxuhsv6ki7r3pm6s7gxc65h52lgqfsa";
        if msg.from==find_addr || msg.to==find_addr {
            println!("Message {}:\n  From {}\n  To {}\n",msg_cid,msg.from,msg.to);
        }
    };
    iterate_over_blockchain(0, 49, &api, Some(on_height), None, None, Some(on_msg), None, None);
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

The crate `indexer` in this repo is another example.  It mines the chain for specific types of messages and publishes them on its HTTP API.

## Configuring Your Lotus

To run these examples, you'll need to repalce `http://lotus1:1234/rpc/v0` with the address of your own Lotus instance.  Set up a Lotus instance like this:

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

