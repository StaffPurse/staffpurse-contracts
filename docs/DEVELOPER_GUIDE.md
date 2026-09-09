# Developer Guide

## Local Environment Setup
1. Install Rust and the wasm32-unknown-unknown target.
2. Install the Stellar CLI: `cargo install --locked stellar-cli`.

## Testnet Deployment
To deploy to the testnet, build the contract and run:
`stellar contract deploy --wasm target/wasm32-unknown-unknown/release/staffpurse_contracts.wasm --network testnet --source <ACCOUNT>`

## Cross-Contract Call References
Contracts can interact with the anchoring contract by importing the compiled WASM and using the generated client.
