#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, Symbol};

fn setup() -> (Env, StaffPurseAnchorClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(StaffPurseAnchor, ());
    let client = StaffPurseAnchorClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    (env, client, admin)
}

#[test]
fn test_initialize_and_anchor_root_success() {
    let (env, client, admin) = setup();
    client.initialize(&admin);

    let batch_date = Symbol::new(&env, "2026-09-09");
    let root = BytesN::from_array(&env, &[1u8; 32]);

    let res = client.try_anchor_root(&batch_date, &root);
    assert!(res.is_ok());

    let stored = client.get_root(&batch_date);
    assert_eq!(stored, Some(root));
}

#[test]
fn test_double_initialize_fails() {
    let (_env, client, admin) = setup();
    client.initialize(&admin);

    let res = client.try_initialize(&admin);
    assert_eq!(res, Err(Ok(ContractError::AlreadyInitialized)));
}

#[test]
fn test_anchor_without_init_fails() {
    let (env, client, _admin) = setup();
    let batch_date = Symbol::new(&env, "2026-09-09");
    let root = BytesN::from_array(&env, &[2u8; 32]);

    let res = client.try_anchor_root(&batch_date, &root);
    assert_eq!(res, Err(Ok(ContractError::NotInitialized)));
}

#[test]
fn test_duplicate_anchor_fails() {
    let (env, client, admin) = setup();
    client.initialize(&admin);

    let batch_date = Symbol::new(&env, "2026-09-09");
    let root1 = BytesN::from_array(&env, &[3u8; 32]);
    let root2 = BytesN::from_array(&env, &[4u8; 32]);

    client.anchor_root(&batch_date, &root1);

    let res = client.try_anchor_root(&batch_date, &root2);
    assert_eq!(res, Err(Ok(ContractError::AlreadyAnchored)));
}

#[test]
fn test_get_nonexistent_root_returns_none() {
    let (env, client, admin) = setup();
    client.initialize(&admin);

    let batch_date = Symbol::new(&env, "2099-01-01");
    assert_eq!(client.get_root(&batch_date), None);
}
