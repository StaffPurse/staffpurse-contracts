use soroban_sdk::{contract, contractimpl, BytesN, Env, Symbol};
use crate::storage::DataKey;

#[contract]
pub struct AnchoringContract;

#[contractimpl]
impl AnchoringContract {
    pub fn get_root(env: Env, batch_date: Symbol) -> Option<BytesN<32>> {
        let key = DataKey::BatchDate(batch_date);
        env.storage().persistent().get(&key)
    }
}
