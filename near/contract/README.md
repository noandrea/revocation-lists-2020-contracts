# A revocation lists contract

### Before you start

Install rust, npx, and npm.

## How to deploy 

> Create contract account

```sh
near create-account revocation-lists.metadid.testnet  --masterAccount metadid.testnet
```
```
Saving key to '$HOME/.near-credentials/testnet/revocation-lists.metadid.testnet.json'
Account revocation-lists.metadid.testnet for network "testnet" was created.
```

> Build contract

```sh
cd contract 
```

```sh
make build
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
```
```
Compiling contract v0.1.0 (...)
   Finished release [optimized] target(s) in 2.12s
```

> Deploy 

```sh
./near deploy --accountId revocation-lists.metadid.testnet --wasmFile target/wasm32-unknown-unknown/release/contract.wasm 
```

```
Starting deployment. Account id: revocation-lists.metadid.testnet, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: target/wasm32-unknown-unknown/release/contract.wasm
Transaction Id 6QaSDXgDBqzw55CP9m5FxkHFzDAgeYvUoHBy3x4eH5qV
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/6QaSDXgDBqzw55CP9m5FxkHFzDAgeYvUoHBy3x4eH5qV
Done deploying to revocation-lists.metadid.testnet
```

> Init the contract

```
near call revocation-lists.metadid.testnet new '{"owner": "metadid.testnet"}'  --accountId metadid.testnet
```

```
Scheduling a call: revocation-lists.metadid.testnet.new({"owner": "metadid.testnet"})
Doing account.functionCall()
Transaction Id DWrvvzt2u8gb3k6NGpyPqTuc3QvuCVr948UdNthX9Ktq
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/DWrvvzt2u8gb3k6NGpyPqTuc3QvuCVr948UdNthX9Ktq
''
```


> Init revocation lists

```
near call revocation-lists.metadid.testnet add_list '{"id": "metadid.testnet/rl/1"}' --accountId metadid.testnet
```

```
Scheduling a call: revocation-lists.metadid.testnet.add_list({"id": "metadid.testnet/rl/1"})
Doing account.functionCall()
Receipt: 4xPa5Nua7fbk3nH2rAuWgwPv1WM7ftDxg4edtyaYXE1F
	Log [revocation-lists.metadid.testnet]: Added a new revocation list
Transaction Id AUS3nE3usr2k9nsCatxcvjGfqYwvuAdUCNBB6qhjhu91
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/AUS3nE3usr2k9nsCatxcvjGfqYwvuAdUCNBB6qhjhu91

```


> Revoke an item

```
./near call revocation-lists.metadid.testnet revoke '{"id": "metadid.testnet/rl/1", "idx": 134}' --accountId metadid.testnet
```

```
Scheduling a call: revocation-lists.metadid.testnet.revoke({"id": "metadid.testnet/rl/1", "idx": 134})
Doing account.functionCall()
Receipt: AYpzRPyFXJme4BZdQBpgmUV4mmekPmQk5CSj16pZyqpU
	Log [revocation-lists.metadid.testnet]: credential updated
Transaction Id 29UGZbFCeJ345QytqQztavV3BN91wJiqRSung2MSu8Sx
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/29UGZbFCeJ345QytqQztavV3BN91wJiqRSung2MSu8Sx
```
> check revocation


```
./near call revocation-lists.metadid.testnet is_revoked '{"id": "metadid.testnet/rl/1", "idx": 134}' --accountId metadid.testnet
```

```
Scheduling a call: revocation-lists.metadid.testnet.is_revoked({"id": "metadid.testnet/rl/1", "idx": 134})
Doing account.functionCall()
Transaction Id HDKTX95hfznazekuR1d8KPUvEsh4QviaHuiuhDH2CzAp
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/HDKTX95hfznazekuR1d8KPUvEsh4QviaHuiuhDH2CzAp
true
```


## Notes

- what about the init method, and what is the role of the owner
- add logic to change the owner




