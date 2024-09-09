# ckb-oracle

This is the oracle contract for Nervos created by Apro.

## Prerequisites

1. Install [ckb-cli](https://github.com/nervosnetwork/ckb-cli). `ckb-cli` is a CKB command line tool, using it to manage accounts and construct transactions

2. Config the network for ckb-cli: `export API_URL=${rpc server}`

| type    | url                         |
| ------- | --------------------------- |
| mainnet | https://mainnet.ckbapp.dev/ |
| mainnet | https://mainnet.ckb.dev/    |
| testnet | https://testnet.ckbapp.dev/ |
| testnet | https://testnet.ckb.dev/    |

3. Generate the address you need. 
```shell
// create an address
ckb-cli account new

// generate the multisign address
ckb-cli tx build-multisig-address \
  --require-first-n 0 \
  --threshold 2 \
  --sighash-address ${address1} \
  --sighash-address ${address2} \
  --sighash-address ${address3} 
  
// get the detail of an address
ckb-cli util address-info --address ${address}
```
4. Acquire the testnet token: https://faucet.nervos.org


## Contract compile and depoly

### contract compile
Run `make build`.

the contract binary code will store in `build` dir.

### Init depoly config
Running `ckb-cli deploy init-config --deployment-config deployment.toml`.

This command will generate the `deployment.toml` in the current dir.  `deployment.toml` defines the meta info for your contract.

More details see: [Deploy-contracts#how-to-use-ckb-cli-to-deploy-a-contract](https://github.com/nervosnetwork/ckb-cli/wiki/Deploy-contracts#how-to-use-ckb-cli-to-deploy-a-contract)

### Generate the deploy transaction
Create a `migration` dir which to store the tx receipt. Running cmd:
```shell
ckb-cli deploy gen-txs \
       --deployment-config ./deployment.toml \
       --migration-dir ./migrations \
       --from-address ${address} \
       --sign-now \
       --info-file info.json
```
Make sure the ${address} has enough CKB. Contract deploy will cost a lot of CKB. 

This cmd will generate the deploy-tx info in the `info.json`.

### Sign tx
```shell
ckb-cli deploy sign-txs \
   --from-account ${address} \
   --add-signatures \
   --info-file info.json
```

### Broadcast tx
```shell
ckb-cli deploy apply-txs --migration-dir ./migrations --info-file info.json
```
This cmd will generate the tx-receipt in the `migrations`. You can find the tx hash in the receipt, 
the contract is considered deployed successfully if the tx is confirmed.