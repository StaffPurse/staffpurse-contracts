use crate::errors::ContractError;
use crate::events::publish_anchored_event;
use crate::storage::DataKey;
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Symbol};

#[contract]
pub struct StaffPurseAnchor;

#[contractimpl]
impl StaffPurseAnchor {
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    pub fn anchor_root(env: Env, batch_date: Symbol, root: BytesN<32>) -> Result<(), ContractError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ContractError::NotInitialized)?;
        
        admin.require_auth();

        let key = DataKey::BatchDate(batch_date.clone());
        if env.storage().persistent().has(&key) {
            return Err(ContractError::AlreadyAnchored);
        }

        env.storage().persistent().set(&key, &root.clone());
        publish_anchored_event(&env, batch_date, root);

        Ok(())
    }

    pub fn get_root(env: Env, batch_date: Symbol) -> Option<BytesN<32>> {
        let key = DataKey::BatchDate(batch_date);
        env.storage().persistent().get(&key)
    }
}
