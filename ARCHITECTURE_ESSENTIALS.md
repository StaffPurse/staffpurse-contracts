# Architecture Essentials

**Repository**: `staffpurse-contracts`
**Purpose**: Soroban smart contract for anchoring daily Merkle roots of StaffPurse spend records.

### 1. Stack
- **Language**: Rust
- **Framework**: `soroban-sdk` (Stellar)

### 2. Core Mechanism
1. **Admin Authorized Write**: A backend service authenticates and calls `anchor_root(batch_date, root)`.
2. **Persistent Storage**: The contract saves `root: BytesN<32>` keyed by `batch_date`.
3. **Public Read**: Anyone can call the read function to retrieve the root for a specific date and verify off-chain proofs.

### 3. Critical Constraints
- **Privacy Enforcement**: Absolutely no raw data on-chain. Only 32-byte hashes are stored.
- **Access Control**: Only the authorized backend service account can write/anchor roots. Public reads are allowed.
- **Storage Strategy**: Use Persistent Storage for the roots, as they must remain accessible indefinitely for historical auditing.
