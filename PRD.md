# Product Requirements Document (PRD): StaffPurse Soroban Anchoring Contracts

## 1. Overview
The StaffPurse Anchoring Contract is a core component of the StaffPurse spend control platform. Since StaffPurse deals with sensitive corporate and employee transaction data, it cannot store raw records on a public blockchain. Instead, to provide an immutable and verifiable audit trail, the platform uses a daily batching process to compute a Merkle tree of all daily spend records and anchors only the Merkle root on the Stellar blockchain (via Soroban).

## 2. Target Audience
- **StaffPurse Backend System**: Will interact with the contract to submit the daily batch root.
- **Third-Party Auditors & StaffPurse Web Dashboard**: Will read the anchored root from the contract to verify individual transaction proofs.

## 3. Core Features & Requirements
- **Anchor Root**: A write function `anchor_root(root: BytesN<32>, batch_date: Symbol)` that securely stores a 32-byte cryptographic hash (the Merkle root) corresponding to a specific day's batch.
- **Read Root**: A read function that allows any party to fetch the anchored `root` for a given `batch_date`.
- **Zero Raw Data**: The contract must strictly deal with cryptographic hashes. No transaction amounts, employee details, or metadata will ever be passed to the contract.
- **Idempotency/Immutability**: A `batch_date` can only be anchored once to prevent historical ledger tampering.

## 4. Authorization & Security
- The contract must restrict the `anchor_root` function to a designated authorized service key (the StaffPurse batching backend). 
- Read access must be completely public.
