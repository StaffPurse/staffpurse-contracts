# 🗺️ ROADMAP.md — `staffpurse-contracts`

> **Scope:** Getting `staffpurse-contracts` approved into the Stellar Wave Program, deploying the Soroban anchoring contract, and running a successful Wave cycle with community contributors.
> **Note:** Wave Program does not fund StaffPurse directly; community contributors earn points by completing backlog issues.

---

## Phase 0 — Feasibility & Ecosystem Prerequisites

Pre-implementation checks and parameter validation before writing Soroban smart contract logic.

### 📋 Current Work
- [x] **Wave Program Acceptance Confirmed:** Stellar Wave Program is active.
- [x] **Personal Org Quotas Checked:** Verified against Tollcraft (3 repos) and Accensa (3 repos) limits.
- [x] **Core Architecture Defined:** Anchoring specification locked (anchor 32-byte cryptographic Merkle roots with date symbols).

### 🔍 Identified Gaps & Action Items (Now Tracked in GitHub Issues)
- [x] **Issue #10 — Accensa Logic Audit & Testnet Funding Spike:** [Audit Accensa contracts for reusable Soroban anchoring patterns and testnet keypair funding](https://github.com/StaffPurse/staffpurse-contracts/issues/10) ✅ *(Completed by @mallison031)*
- [x] **Network & Keypair Strategy:** Finalize testnet standalone account vs shared testnet RPC keypair funding (`Friendbot` keypair management).
- [x] **Points Allocation Budget:** Confirm exact Wave 1 points ceiling for `staffpurse-contracts` on the Drips dashboard (target: ~25,000 pts).

---

## Phase 1 — Repository Readiness & Contract Implementation

Requirements to bring the repository to an institutional, auditable state that satisfies Stellar Wave maintainer review.

### 📋 Current Work (Tracked in GitHub Issues)
- [x] **Issue #1 — Repository Hygiene & Security:** [Add SECURITY.md and configure repository hygiene](https://github.com/StaffPurse/staffpurse-contracts/issues/1) ✅ *(Completed by @mallison031)*
- [x] **Issue #2 — Storage Keys & Data Models:** [Define Soroban storage keys and core data structures](https://github.com/StaffPurse/staffpurse-contracts/issues/2)
- [x] **Issue #3 — Core Write Function:** [Implement anchor_root write function with admin auth](https://github.com/StaffPurse/staffpurse-contracts/issues/3) ✅ *(Completed by @mallison031)*
- [x] **Issue #4 — Core Read Function:** [Implement get_root public read function](https://github.com/StaffPurse/staffpurse-contracts/issues/4)
- [ ] **Issue #5 — Test Suite:** [Create unit tests for anchor and read flows](https://github.com/StaffPurse/staffpurse-contracts/issues/5)
- [x] **Issue #6 — Custom Errors & Events:** [Define custom ContractError enums and Soroban Events](https://github.com/StaffPurse/staffpurse-contracts/issues/6)
- [x] **Issue #7 — Local Deployment Script:** [Write local deployment and initialization shell script](https://github.com/StaffPurse/staffpurse-contracts/issues/7) ✅ *(Completed by @mallison031)*
- [x] **Issue #8 — External Documentation Content:** [Write GitBook external documentation content](https://github.com/StaffPurse/staffpurse-contracts/issues/8)
- [x] **Issue #11 — Project Scaffolding:** [Initialize Cargo workspace and Soroban contract crate structure](https://github.com/StaffPurse/staffpurse-contracts/issues/11)
- [x] **Issue #12 — GitHub Actions CI Pipeline:** [Set up GitHub Actions CI for Cargo test, clippy, and fmt](https://github.com/StaffPurse/staffpurse-contracts/issues/12) ✅ *(Completed by @mallison031)*
- [x] **Issue #13 — Release Tagging & Deployment Guide:** [Create testnet release tagging workflow and deployment documentation](https://github.com/StaffPurse/staffpurse-contracts/issues/13)

### 🔍 Identified Gaps & Action Items
- [x] ~~**GAP-C1: Project Scaffolding (`Cargo.toml` & Crate Structure)**~~ → Created as **Issue #11**
- [x] ~~**GAP-C2: GitHub Actions CI Pipeline (`.github/workflows/ci.yml`)**~~ → Created as **Issue #12**
- [x] ~~**GAP-C3: Release Tagging (`v0.1.0`) & Guide**~~ → Created as **Issue #13**
- [ ] **GAP-C4: GitBook Site Hosting:** Link `docs/` or setup GitBook sync to provide a live documentation URL for Wave reviewers.

---

## Phase 3 — Wave Issue Backlog & Point Sizing

Preparing a production-grade issue backlog for Wave contributors.

### 📋 Current Work
- [x] Standard Drips Wave issue template created in `.github/ISSUE_TEMPLATE/drips-wave-issue.md`.
- [x] Initial Phase 1 issues (#1–#8) published with structured context, acceptance criteria, and quality standards.

### 🔍 Identified Gaps & Action Items
- [ ] **GAP-C5: GitHub Labels Configuration:**
  - *Problem:* Repository only has standard GitHub labels (`bug`, `enhancement`). Wave complexity labels are absent.
  - *Action:* Create labels:
    - `complexity: trivial (100 pts)`
    - `complexity: medium (150 pts)`
    - `complexity: high (200 pts)`
    - `wave-1`
- [ ] **GAP-C6: Issue Sizing & Point Assignment:**
  - *Problem:* Current open issues do not reflect point values in their titles or labels.
  - *Action:* Tag existing and future issues:
    - `#1` Repo Hygiene → Trivial (100 pts)
    - `#2` Storage Keys → Medium (150 pts)
    - `#3` Anchor Root → High (200 pts)
    - `#4` Get Root → Trivial (100 pts)
    - `#5` Unit Tests → High (200 pts)
    - `#6` Errors & Events → Medium (150 pts)
    - `#7` Deploy Script → Medium (150 pts)
    - `#8` GitBook Docs → High (200 pts)
- [ ] **GAP-C7: Wave 2 Feature Backlog Seeding:**
  - *Problem:* If Wave 1 finishes quickly, there is no second tier of issues ready.
  - *Action:* Draft Wave 2 candidate issues:
    - Multi-signature / timelock admin upgradeability.
    - Historical root pagination & batch enumeration (`get_roots(start, limit)`).
    - Soroban fee and resource optimization analysis.

---

## Phase 4 — Wave 1 Operational Execution

Managing contributors during the 1-week Wave execution cycle.

### 📋 Current Work
- [x] Contact links (Telegram & Discord) added to issue guidelines.
- [x] Code quality guidelines outlined in `CONTRIBUTING.md`.

### 🔍 Identified Gaps & Action Items
- [ ] **GAP-C8: Pull Request Template (`.github/PULL_REQUEST_TEMPLATE.md`):**
  - *Problem:* No PR template to enforce mandatory checklist items (e.g. CI passing, `Closes #X`, test execution proof).
  - *Action:* Create PR template with standard verification steps.
- [ ] **GAP-C9: Contributor Assignment Stale Policy:**
  - *Problem:* Contributors may claim an issue and go inactive, blocking others during the 1-week wave.
  - *Action:* Establish a 48-hour activity SLA before unassigning inactive contributors.
- [ ] **GAP-C10: Issue Template Placeholder Cleanup:**
  - *Problem:* `.github/ISSUE_TEMPLATE/drips-wave-issue.md` still contains placeholder URLs (`[link]`, `$org/$repo`).
  - *Action:* Update template with literal StaffPurse links.

---

## Phase 5 — Iteration & Wave Closeout

Post-cycle review, point distribution, and backlog maintenance.

### 📋 Current Work
- [ ] Retrospective cadence defined.

### 🔍 Identified Gaps & Action Items
- [ ] **GAP-C11: Drips Attestation Workflow:** Document maintainer steps to verify PR merge and approve point attestations on Drips within the 14-day window.
- [ ] **GAP-C12: Unfinished PR Triage:** Policy to salvage, re-open, or squash-merge incomplete PRs when the cycle closes.
- [ ] **GAP-C13: Budget Reconciliation:** Track actual points burned against the 25k allocation to size Wave 2 correctly.

---

## Open Decisions & Technical Risks
1. **Contract Upgradeability:** Should the initial contract use `env.deployer().update_current_contract_wasm()` or remain immutable for MVP transparency?
2. **Batch Identifier Format:** `Symbol` (e.g., `Symbol::new(&env, "2026-09-09")`) vs `u64` Unix day timestamp.
3. **Storage Tiering:** Using `Instance` vs `Persistent` storage for root records to optimize TTL extension costs on Stellar testnet/mainnet.
