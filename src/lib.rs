#![no_std]

pub mod errors;
pub mod events;
pub mod storage;

#[cfg(test)]
mod test;

use errors::ContractError;
use events::emit_anchor_root;
use storage::DataKey;

use soroban_sdk::{contract, contractimpl, contractmeta, Address, BytesN, Env, Symbol};

contractmeta!(key = "name", val = "StaffPurseAnchor");
contractmeta!(key = "version", val = env!("CARGO_PKG_VERSION"));
contractmeta!(
    key = "repo",
    val = "https://github.com/StaffPurse/staffpurse-contracts"
);

/// Approximately 30 days of ledgers (~5s per ledger).
/// 60 * 60 * 24 * 30 / 5 = 518,400 ledgers.
const TTL_EXTEND: u32 = 518_400;
/// Threshold before extending TTL.
const TTL_THRESHOLD: u32 = 100;

#[contract]
pub struct StaffPurseAnchor;

#[contractimpl]
impl StaffPurseAnchor {
    /// Initializes the contract with an authorized admin address.
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND);

        Ok(())
    }

    /// Anchors a 32-byte cryptographic Merkle root for a given batch date.
    ///
    /// Requires authorization from the contract's registered admin address.
    /// Rejects attempts to overwrite an already anchored batch date.
    pub fn anchor_root(
        env: Env,
        batch_date: Symbol,
        root: BytesN<32>,
    ) -> Result<(), ContractError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ContractError::NotInitialized)?;

        admin.require_auth();

        let key = DataKey::BatchRoot(batch_date.clone());
        if env.storage().persistent().has(&key) {
            return Err(ContractError::AlreadyAnchored);
        }

        env.storage().persistent().set(&key, &root);
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND);
        env.storage()
            .instance()
            .extend_ttl(TTL_THRESHOLD, TTL_EXTEND);

        emit_anchor_root(&env, batch_date, root);

        Ok(())
    }

    /// Reads an anchored Merkle root for a given batch date, if it exists.
    pub fn get_root(env: Env, batch_date: Symbol) -> Option<BytesN<32>> {
        let key = DataKey::BatchRoot(batch_date);
        env.storage().persistent().get(&key)
    }
}
