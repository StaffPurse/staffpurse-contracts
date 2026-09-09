# Security Policy

## Threat Model & Security Properties

StaffPurse contracts provide an immutable, auditable anchoring mechanism on the Stellar blockchain (Soroban). For architectural specifications and data structures, consult [ARCHITECTURE.md](ARCHITECTURE.md) and [PRD.md](PRD.md).

## Supported Versions

Only the latest `main` branch and active tagged releases are supported with security updates.

| Version | Supported |
| ------- | --------- |
| `main` (`v0.1.x`) | :white_check_mark: |
| `< 0.1.0` | :x: |

## Reporting a Vulnerability

If you discover a security vulnerability in the Soroban smart contracts, **do not report it publicly** via GitHub issues, pull requests, or public discussions.

### Preferred Reporting Channel
Use **[GitHub Private Vulnerability Reporting](https://github.com/StaffPurse/staffpurse-contracts/security/advisories/new)** to submit confidential security advisories directly to the maintainers.

### Secondary Contact
If private advisory submission is unavailable, reach out directly:
- **Telegram:** Maintainers direct contact in [StaffPurse Group](https://t.me/+Gflo5jZStw1jMjE0)
- **Discord:** Direct message core maintainers in [StaffPurse Server](https://discord.gg/5aprtMSyR)

## Scope

### In-Scope
- `StaffPurseAnchor` Soroban smart contract logic.
- Storage keys, TTL management, and state persistence in Soroban instance/persistent storage.
- Authorization checks on `anchor_root` ensuring only designated admin address can write roots.
- Error codes and event emissions for indexer integrity.
- Verified testnet and mainnet contract deployments.

### Out-of-Scope
- Client-side key custody (Edge Function environment variables or user wallets).
- Web verification dashboard (covered in [`staffpurse-web`](https://github.com/StaffPurse/staffpurse-web)).
- Mobile application client (covered in [`staffpurse-app`](https://github.com/StaffPurse/staffpurse-app)).
- Upstream Stellar core consensus or Soroban host environment vulnerabilities.

## Response SLA & Disclosure Policy

- **Initial Triage:** Maintainers will acknowledge and assess report severity within **48 hours**.
- **Status Updates:** Progress updates provided every **5 business days** during active remediation.
- **Coordinated Disclosure:** We follow a standard **90-day coordinated disclosure timeline** before public advisory publication.

> [!NOTE]
> The smart contracts in this repository are currently in **testnet/pre-audit stage**. Exercise diligence when deploying to production environments.
