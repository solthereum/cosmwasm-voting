# CosmWasm Voting Smart Contract

CosmWasm template is used to build voting smart contract in Rust to run inside a
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

http://rpc.oysternet.cosmwasm.com/status
https://faucet.oysternet.cosmwasm.com/status
http://lcd.oysternet.cosmwasm.com/node_info

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
$ JSON=$(jq -n --arg addr $(wasmd keys show -a fred) '{"denom":"usponge" "address":$addr}') && curl -X POST --header "Content-Type: application/json" --data "$JSON" https://faucet.oysternet.cosmwasm.com/credit
$ JSON=$(jq -n --arg addr $(wasmd keys show -a thief) '{"denom":"usponge" "address":$addr}') && curl -X POST --header "Content-Type: application/json" --data "$JSON" https://faucet.oysternet.cosmwasm.com/credit
```

**Export wasmd Parameters.**

If you intend to use wasmd as client, we recommend you to setup these variables. Otherwise You will have to define type in node, chain id and gas-prices details with every command you execute

```
$ export NODE="--node $RPC"
$ export TXFLAG="${NODE} --chain-id ${CHAIN_ID} --gas-prices 0.001usponge --gas auto --gas-adjustment 1.3"
```
## Creating a new repo from template

Assuming you have a recent version of rust and cargo (v1.51.0+) installed
(via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

First, install
[cargo-generate](https://github.com/ashleygwilliams/cargo-generate).
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:


**0.14 (latest)**

```sh
cargo generate --git https://github.com/CosmWasm/cosmwasm-template.git --name PROJECT_NAME
```