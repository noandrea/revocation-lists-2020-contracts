package main

import (
	"encoding/hex"
	"fmt"
	"io/ioutil"
	"math/big"
	"time"

	"github.com/casper-ecosystem/casper-golang-sdk/keypair/ed25519"
	"github.com/casper-ecosystem/casper-golang-sdk/sdk"
)

var (
	nodeRpc                 = "http://135.181.208.231:7777/rpc"
	nodeEvent               = "http://135.181.208.231:9999"
	privKeyPath             = "../_private/secret_key.pem"
	pubKeyHex               = "01cded33d09474d1c9c9039e8af42b5c8f04e35b47c289f29f89ce3bc8fb03084c"
	modulePath              = "../contract/target/wasm32-unknown-unknown/release/contract.wasm"
	chainName               = "casper-test"
	deployPaymentCSPR int64 = 35
)

func main() {

	//deploy()
	call()
	
}

func call() {
	rpcClient := sdk.NewRpcClient(nodeRpc)
	eventClient := sdk.NewEventService(nodeEvent)

	pkb, _ := hex.DecodeString(pubKeyHex)

	skb, _ := ed25519.ParsePrivateKeyFile(privKeyPath)
	pair := ed25519.ParseKeyPair(pkb, skb)

	deployParams := sdk.NewDeployParams(pair.PublicKey(), chainName, [][]uint8{}, time.Now().UnixMilli())

	payment := sdk.StandardPayment(big.NewInt(1 * 1_000_000_000))

	deployHash := "d5c3b9524fc64012d54440173fb6dd2d29828fdebfa6cea5c533a44216b959fd"
	dhb, _ := hex.DecodeString(deployHash)
	var dhb32 [32]byte
	copy(dhb32[:], dhb)
	session := sdk.NewStoredContractByHash(dhb32, "get_funds_raised", *sdk.NewRunTimeArgs(map[string]sdk.Value{}, []string{}))

	deploy := sdk.MakeDeploy(deployParams, payment, session)
	deploy.SignDeploy(pair)

	putDeploy, err := rpcClient.PutDeploy(*deploy)
	if err != nil {
		fmt.Printf("PutDeploy error: %v\n", err)
		return
	}
	fmt.Printf("deploy hash: %v\n", putDeploy.Hash)
	fmt.Printf("PutDeploy %#v\n", putDeploy)

	d, err := rpcClient.GetDeploy(putDeploy.Hash)
	if err != nil {
		fmt.Printf("GetDeploy error: %v\n", err)
		return
	}
	fmt.Printf("GetDeploy %#v\n", d)

	processedDeploy, err := eventClient.GetDeployByHash(putDeploy.Hash)
	if err != nil {
		fmt.Printf("GetDeployByHash error: %v\n", err)
		return
	}

	fmt.Printf("%v\n", processedDeploy)
}

func deploy() {
	rpcClient := sdk.NewRpcClient(nodeRpc)
	eventClient := sdk.NewEventService(nodeEvent)

	pkb, _ := hex.DecodeString(pubKeyHex)

	skb, _ := ed25519.ParsePrivateKeyFile(privKeyPath)
	pair := ed25519.ParseKeyPair(pkb, skb)

	module, _ := ioutil.ReadFile(modulePath)

	deployParams := sdk.NewDeployParams(pair.PublicKey(), chainName, [][]uint8{}, time.Now().UnixMilli())
	payment := sdk.StandardPayment(big.NewInt(deployPaymentCSPR * 1_000_000_000))
	session := sdk.NewModuleBytes(module, *sdk.NewRunTimeArgs(map[string]sdk.Value{}, []string{}))

	deploy := sdk.MakeDeploy(deployParams, payment, session)
	deploy.SignDeploy(pair)

	putDeploy, _ := rpcClient.PutDeploy(*deploy)
	fmt.Printf("deploy hash: %v\n", putDeploy.Hash)

	processedDeploy, _ := eventClient.GetDeployByHash(putDeploy.Hash)

	fmt.Printf("%+v\n", processedDeploy)
}
