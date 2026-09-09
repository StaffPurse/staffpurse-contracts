# Spike Report: Accensa Contracts Audit & Reusable Patterns

> **Issue:** [staffpurse-contracts#10](https://github.com/StaffPurse/staffpurse-contracts/issues/10)  
> **Author:** `@mallison031`  
> **Status:** Complete  
> **Source Reference:** `Accensa/accensa-contracts`

---

## Executive Summary

An architectural audit of `Accensa/accensa-contracts` was conducted to identify reusable Soroban patterns for `staffpurse-contracts`. The Accensa codebase includes a production-grade anchoring system (`ReceiptAnchor` at `contracts/receipt-anchor`) with clean authorization, storage schemas, event emissions, and automated testnet deployment pipelines. 

While Accensa implements advanced sharding (`ReceiptShard`) and ZK verification, StaffPurse's transparency layer requires a streamlined, single-contract pattern (`anchor_root(batch_date, root)`). However, Accensa's foundational modules provide directly portable templates.

---

## 1. Reusable Soroban Code Patterns

### A. Storage Key Schema (`storage.rs`)
Accensa structures contract state cleanly using `#[contracttype]` enums:
```rust
#[contracttype]
pub enum DataKey {
    Admin,
    BatchRoot(Symbol),        // Direct mapping: BatchDate Symbol -> 32-byte Merkle root
    BatchTimestamp(Symbol),   // Unix ledger timestamp when anchored
    BatchCount,               // Total number of batches anchored
}
```
**Takeaway for StaffPurse:**
- Store roots as `BytesN<32>` keyed by `Symbol` (`DataKey::BatchRoot(batch_date)`).
- Use `env.storage().instance()` for contract metadata (Admin, TotalBatches) and `env.storage().persistent()` for historical batch roots with appropriate TTL extensions.

### B. Authorization & Admin Verification
Accensa enforces admin authorization using native Soroban checks:
```rust
let admin: Address = env
    .storage()
    .instance()
    .get(&DataKey::Admin)
    .ok_or(ContractError::NotInitialized)?;

admin.require_auth();
```
**Takeaway for StaffPurse:**
- Require admin authentication on `anchor_root` to ensure only the authorized StaffPurse backend service keypair can submit roots.
- Reject attempts to overwrite an already anchored batch date (`ContractError::AlreadyAnchored`).

### C. Soroban Events (`events.rs`)
Accensa publishes structured contract events on state changes:
```rust
#[contracttype]
pub struct BatchAnchoredEvent {
    pub batch_date: Symbol,
    pub root: BytesN<32>,
    pub timestamp: u64,
}

env.events().publish(
    (Symbol::new(&env, "anchor"), batch_date),
    BatchAnchoredEvent { batch_date, root, timestamp }
);
```
**Takeaway for StaffPurse:**
- Publish `(Symbol::new(&env, "anchor_root"), batch_date)` events so indexers and the web dashboard can stream updates.

### D. Contract Metadata
Accensa uses `contractmeta!` for reproducibility and audit traceability:
```rust
contractmeta!(key = "name", val = "StaffPurseAnchor");
contractmeta!(key = "version", val = env!("CARGO_PKG_VERSION"));
contractmeta!(key = "repo", val = "https://github.com/StaffPurse/staffpurse-contracts");
```

---

## 2. Testnet Keypair Funding & Deployment Automation

### A. Stellar CLI Deployment Pipeline (`deploy.sh`)
Accensa's `deploy.sh` script provides a robust deployment recipe:
1. **Network Identity:** Uses Stellar CLI identity (`stellar keys generate deployer --network testnet`).
2. **Automated Funding:** Calls Friendbot automatically:
   ```bash
   curl -s "https://friendbot.stellar.org?addr=$(stellar keys address deployer)"
   ```
3. **Optimized Compilation:** Builds release WASM:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   stellar contract optimize --wasm target/wasm32-unknown-unknown/release/staffpurse_anchor.wasm
   ```
4. **Contract Installation & Initialization:**
   ```bash
   CONTRACT_ID=$(stellar contract deploy \
     --wasm target/wasm32-unknown-unknown/release/staffpurse_anchor.optimized.wasm \
     --source deployer \
     --network testnet)
   
   stellar contract invoke \
     --id "$CONTRACT_ID" \
     --source deployer \
     --network testnet \
     -- initialize \
     --admin "$(stellar keys address deployer)"
   ```
5. **Committed Trail:** Writes outputs to `deployments/testnet.env`:
   ```bash
   CONTRACT_ID=C...
   DEPLOYER_ADDRESS=G...
   NETWORK=testnet
   ```

---

## 3. Recommended Implementation Plan for StaffPurse

1. **Adopt Crate Layout:** Create a single contract crate `contracts/anchor` with modules `storage.rs`, `errors.rs`, `events.rs`, `lib.rs`, and `test.rs`.
2. **Reuse Deployment Script:** Adapt Accensa's `deploy.sh` for `staffpurse-contracts` with automated Friendbot funding and `deployments/testnet.env` tracking.
3. **Avoid Over-Engineering:** Omit Accensa's sharding and zero-knowledge verification rings; StaffPurse needs simple, robust Merkle root anchoring only.
