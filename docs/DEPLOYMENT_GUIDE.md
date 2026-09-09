# Deployment Guide

## Building the Contract
1. Build the contract to WebAssembly:
   `stellar contract build`

## Deploying to Testnet
1. Ensure your network is configured for Testnet.
2. Deploy the compiled WASM file using the Stellar CLI:
   `stellar contract deploy --wasm target/wasm32-unknown-unknown/release/staffpurse_contracts.wasm --source alice --network testnet`
