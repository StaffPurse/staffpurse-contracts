# Introduction

The transparency layer solves the problem of unverifiable spend records by anchoring Merkle roots of transaction batches to the Stellar network. It ensures data immutability without compromising privacy.

## How it works
1. Off-chain spend records are hashed.
2. The hashes are structured into a Merkle tree.
3. The Merkle root is anchored on-chain via the Soroban contract.
