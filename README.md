<div align="center">
  <!-- 🖼️ Banner/Logo Placeholder -->
  <img src="https://via.placeholder.com/800x200/1e1e2e/a6accd?text=StaffPurse+staffpurse-contracts" alt="" width="100%" />

  <h1>StaffPurse Contracts</h1>
  <p><strong>Soroban smart contracts for StaffPurse daily Merkle-root batching.</strong></p>

  <p>
    <img src="https://img.shields.io/github/actions/workflow/status/StaffPurse/staffpurse-contracts/rust-ci.yml?branch=main" alt="CI Status" />
    <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License" />
  </p>

  <p>
    <a href="https://staffpurse.gitbook.io"><strong>Documentation</strong></a> ·
    <a href="https://t.me/+Gflo5jZStw1jMjE0"><strong>Community Telegram</strong></a>
  </p>
</div>

## 📖 Overview

The on-chain transparency layer for StaffPurse. By anchoring daily Merkle roots of off-chain spend records, we enable cryptographic, third-party auditing without exposing sensitive business or employee data to a public ledger.

## 🏗 Architecture

Written in **Rust** using the `soroban-sdk`. The contract acts as an immutable key-value store. It exposes an `anchor_root` function strictly authenticated for the admin backend, and a public `get_root` function for verification queries.

## 🚀 Quick Start

```bash
# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Build the Soroban contract
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test
```

## 🤝 Contributing

Please read our [Contributing Guidelines](CONTRIBUTING.md) and [Security Policy](SECURITY.md) before submitting pull requests. All PRs must pass the CI gates and follow our code quality standards.

## 👥 Maintainers

| Name | Contact | Role |
| :--- | :--- | :--- |
| Ademola | [Telegram](https://t.me/placeholder) | Core Maintainer |

## ✨ Contributors

<a href="https://github.com/StaffPurse/staffpurse-contracts/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=StaffPurse/staffpurse-contracts" alt="Contributors" />
</a>
