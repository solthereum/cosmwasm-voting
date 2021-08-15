# CosmWasm Voting Smart Contract

CosmWasm template is used to build Voting Smart Contract in Rust to run inside a
[Cosmos SDK](https://github.com/cosmos/cosmos-sdk) module on all chains that enable it.

## Common setup

**Clone the voting smart contract repo.**

```
$ git clone https://github.com/shaikrasheed99/cosmwasm-voting-smartcontract.git
$ cd cosmwasm-voting-smartcontract
```

**Install dependencies.**

Wasmd is the backbone of CosmWasm platform. It is the implementation of a Cosmoszone with wasm smart contracts enabled. To edit or run a contract, you need wasmd.

```
$ git clone https://github.com/CosmWasm/wasmd.git
$ cd wasmd
$ git checkout v0.16.0
$ make install
```

**Verify the wasmd installation.**
```
$ wasmd version
```

## Compile the wasm contract with stable toolchain

```
$ rustup default stable
$ cargo wasm
```

To produce a much smaller version, you can run this which tells the compiler to strip all unused code out.

```
$ RUSTFLAGS='-C link-arg=-s' cargo wasm
```

## Setting Up Environment

You need an environment to run contracts. You can either run your node locally or connect to an existing network. For easy testing, oysternet network is online, you can use it to deploy and run your contracts.

**To verify testnet is currently running, make sure the following URLs are all working for you:**

 - http://rpc.oysternet.cosmwasm.com/status
 - https://faucet.oysternet.cosmwasm.com/status
 - http://lcd.oysternet.cosmwasm.com/node_info

## Setup Go CLI

Let's configure wasmd exec, point it to testnets, create wallet and ask tokens from faucet.

**First source the oysternet network configurations to the shell:**

```
$ source <(curl -sSL https://raw.githubusercontent.com/CosmWasm/testnets/master/oysternet-1/defaults.env)
```

**Setup the client:**

```
$ wasmd keys add fred
$ wasmd keys add bob
```

**Requesting tokens from faucet, incase if you ran out of tokens:**

```
$ JSON = $(jq -n --arg addr $(wasmd keys show -a fred) '{"denom":"usponge" "address":$addr}') && curl -X POST --header "Content-Type: application/json" --data "$JSON" https://faucet.oysternet.cosmwasm.com/credit
$ JSON = $(jq -n --arg addr $(wasmd keys show -a thief) '{"denom":"usponge" "address":$addr}') && curl -X POST --header "Content-Type: application/json" --data "$JSON" https://faucet.oysternet.cosmwasm.com/credit
```

**Export wasmd Parameters.**

If you intend to use wasmd as client, we recommend you to setup these variables. Otherwise You will have to define type in node, chain id and gas-prices details with every command you execute

```
$ export NODE = "--node $RPC"
$ export TXFLAG = "${NODE} --chain-id ${CHAIN_ID} --gas-prices 0.001usponge --gas auto --gas-adjustment 1.3"
```

**See how many codes we have now.**

```
$ wasmd query wasm list-code $NODE
```

**Gas is huge due to wasm size, but auto-zipping reduced this from 1.8M to around 600k**

```
$ RES = $(wasmd tx wasm store target/wasm32-unknown-unknown/release/voting.wasm --from fred $TXFLAG -y)
```

**You can also get the code this way.**

```
$ CODE_ID = $(echo $RES | jq -r '.logs[0].events[0].attributes[-1].value')
```

**No contracts yet, this should return an empty list**

```
$ wasmd query wasm list-contract-by-code $CODE_ID $NODE --output json
```

## Instantiating the Contract

**Instantiate contract and verify**

```
$ wasmd tx wasm instantiate $CODE_ID "{}" --from fred --label "voting 1" $TXFLAG -y
```

**Check the contract state**

```
$ wasmd query wasm list-contract-by-code $CODE_ID $NODE --output json
$ CONTRACT = $(wasmd query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')
```

## Executing the Contract features

**Create a single voter**

```
$ CREATE = $(jq -n --arg bob $(wasmd keys show -a bob) '{"create_voter":{"address":$bob,"name":"bob"}}')
$ wasmd tx wasm execute  $CONTRACT "$CREATE" --from fred $TXFLAG -y
```

**Registering a voter as a candidate**

```
$ REG = $(jq -n --arg bob $(wasmd keys show -a bob) '{"register":{"voter":$bob}}')
$ wasmd tx wasm execute  $CONTRACT "$REG" --from fred $TXFLAG -y
```

**Casting vote to a particular candidate**

```
$ CASTVOTE = $(jq -n --arg bob $(wasmd keys show -a bob) --arg fred $(wasmd keys show -a fred) '{"cast_vote":{"voter":$fred,"candidate":$bob}}')
$ wasmd tx wasm execute  $CONTRACT "$CASTVOTE" --from fred $TXFLAG -y
```

## Querying the Contract details

**To find how many number of voters inside contract**

```
$ wasmd query wasm contract-state smart $CONTRACT '{"voters": {}}' $NODE
```

**To find how many number of candidates inside contract**

```
$ wasmd query wasm contract-state smart $CONTRACT '{"candidates": {}}' $NODE
```

**To find how many number of casted votes of a particular candidate inside contract**

```
$ VOTES = $(jq -n --arg bob $(wasmd keys show -a bob) '{"votes":{"candidate":$bob}}')
$ wasmd query wasm contract-state smart $CONTRACT "$VOTES1" $NODE
```
