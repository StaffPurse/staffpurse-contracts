# Deployment Guide

## Building the Contract
1. Build the contract to WebAssembly:
   `stellar contract build`

## Deploying to Testnet
1. Ensure your network is configured for Testnet.
2. Deploy the compiled WASM file using the Stellar CLI:
   `stellar contract deploy --wasm target/wasm32-unknown-unknown/release/staffpurse_contracts.wasm --source alice --network testnet`

## Network & Keypair Strategy
For Testnet deployments and testing, we have finalized a **Standalone Account Strategy** rather than a shared RPC keypair:
- **How it works**: Each deployment script execution (via `deploy.sh`) or CI run generates its own isolated Stellar identity.
- **Funding**: The identity is automatically funded via the Stellar Friendbot API prior to contract deployment.
- **Why**: This prevents sequence number collisions that commonly occur when multiple developers or parallel CI jobs attempt to use a single shared keypair. It ensures deterministic deployment and testing behavior.
