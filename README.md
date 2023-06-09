# Matrix AI

A fresh [MatrixAI](https://substrate.io/) Chain node, ready for hacking :rocket:

## Getting Started

Depending on your operating system and Rust version, there might be additional packages required to compile this project.
Check the [installation](https://docs.substrate.io/install/) instructions for your platform for the most common dependencies.

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Single-Node Development Chain

The following command starts a single-node development chain that doesn't persist state:

```sh
./target/release/matrix-ai --dev
```

To purge the development chain's state, run the following command:

```sh
./target/release/matrix-ai purge-chain --dev
```

To start the development chain with detailed logging, run the following command:

```sh
RUST_BACKTRACE=1 ./target/release/matrix-ai -ldebug --dev
```

Development chains:

- Maintain state in a `tmp` folder while the node is running.
- Use the **Alice** and **Bob** accounts as default validator authorities.
- Use the **Alice** account as the default `sudo` account.
- Are preconfigured with a genesis state (`/node/src/chain_spec.rs`) that includes several prefunded development accounts.

To persist chain state between runs, specify a base path by running a command similar to the following:

```sh
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/matrix-ai --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```

### Connect with Polkadot-JS Apps Front-End

After you start the node locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint.
A hosted version is also available on [IPFS (redirect) here](https://dotapps.io/) or [IPNS (direct) here](ipns://dotapps.io/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer).
You can also find the source code and instructions for hosting your own instance on the [polkadot-js/apps](https://github.com/polkadot-js/apps) repository.