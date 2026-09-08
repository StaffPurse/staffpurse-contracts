# Architecture & Tech Stack

## 1. System Architecture
This repository contains the on-chain Soroban smart contracts for StaffPurse's transparency layer. The architecture relies on an off-chain/on-chain split to achieve verifiable immutability without sacrificing privacy.

- **Off-Chain (Backend)**: Daily cron jobs compile all new spend records, compute a Merkle tree, and submit the resulting root hash to this contract.
- **On-Chain (This Contract)**: Acts as a decentralized, immutable key-value store mapping dates to Merkle roots.

## 2. Tech Stack
- **Language**: Rust
- **Blockchain/Framework**: Stellar network using the `soroban-sdk`.
- **Testing**: Native Rust unit tests and Soroban test utilities.

## 3. Data Models
### Contract Storage
- **Data Type**: Persistent Storage.
- **Key**: `batch_date` (represented as a `Symbol` or `String` depending on character limits, typically a format like `YYYYMMDD`).
- **Value**: `root` (represented as `BytesN<32>`).

## 4. Contract Interfaces
### `anchor_root`
- **Inputs**: 
  - `batch_date: Symbol`
  - `root: BytesN<32>`
- **Modifiers**: Requires authentication from the admin/service address.
- **Behavior**: Stores the root in persistent storage under the `batch_date` key.

### `get_root`
- **Inputs**: 
  - `batch_date: Symbol`
- **Outputs**: 
  - `Option<BytesN<32>>`
- **Behavior**: Returns the anchored root if it exists, otherwise returns `None`.
