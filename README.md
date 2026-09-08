# StaffPurse Contracts

> On-chain anchoring and transparency for the StaffPurse spend control platform.

This repository contains the Soroban smart contracts that serve as the transparency layer for StaffPurse. By anchoring daily Merkle roots of off-chain spend records, we enable cryptographic, third-party auditing without ever exposing sensitive business or employee data to a public ledger.

---

## 🛠 Tech Stack

- **Language:** Rust
- **Blockchain Framework:** Stellar / Soroban (`soroban-sdk`)
- **Testing:** Rust native unit testing and Soroban test utilities

---

## 🚀 Getting Started

Because these are Soroban smart contracts, you need the Rust toolchain and the WebAssembly target installed, along with the Stellar CLI for local deployment and testing.

### 1. Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- The WebAssembly target: `rustup target add wasm32-unknown-unknown`
- [Stellar CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup) for deploying and invoking contracts.

### 2. Build the Contract
Compile the contract to WebAssembly:
```bash
cargo build --target wasm32-unknown-unknown --release
```

### 3. Run the Tests
Execute the native Rust unit tests to verify the anchoring logic and access controls:
```bash
cargo test
```

---

## ⚓ Anchoring & Auditing

The contract acts as a decentralized, immutable key-value store mapping dates to Merkle roots.
- **`anchor_root(batch_date: Symbol, root: BytesN<32>)`**: Called exclusively by the authorized StaffPurse backend cron job to securely store a 32-byte cryptographic hash (the daily Merkle root).
- **`get_root(batch_date: Symbol)`**: A public read function that allows any auditor or dashboard (like `staffpurse-web`) to fetch the anchored root for a specific day and verify individual transaction proofs.

---

## 🏗 Architecture Reference
- Read [ARCHITECTURE.md](ARCHITECTURE.md) for data flow and structural decisions.
- Read [ARCHITECTURE_ESSENTIALS.md](ARCHITECTURE_ESSENTIALS.md) for a quick overview of critical constraints.
- Read [PRD.md](PRD.md) for product scope and targeted use cases.
