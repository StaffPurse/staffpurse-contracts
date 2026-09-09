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

    let batch_date = Symbol::new(&env, "d20260909");
    let root = BytesN::from_array(&env, &[1u8; 32]);

    client.anchor_root(&batch_date, &root);

    let stored = client.get_root(&batch_date);
    assert_eq!(stored, Some(root));
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_double_initialize_fails() {
    let (_env, client, admin) = setup();
    client.initialize(&admin);
    client.initialize(&admin);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_anchor_without_init_fails() {
    let (env, client, _admin) = setup();
    let batch_date = Symbol::new(&env, "d20260909");
    let root = BytesN::from_array(&env, &[2u8; 32]);

    client.anchor_root(&batch_date, &root);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_duplicate_anchor_fails() {
    let (env, client, admin) = setup();
    client.initialize(&admin);

    let batch_date = Symbol::new(&env, "d20260909");
    let root1 = BytesN::from_array(&env, &[3u8; 32]);
    let root2 = BytesN::from_array(&env, &[4u8; 32]);

    client.anchor_root(&batch_date, &root1);
    client.anchor_root(&batch_date, &root2);
}

#[test]
fn test_get_nonexistent_root_returns_none() {
    let (env, client, admin) = setup();
    client.initialize(&admin);

    let batch_date = Symbol::new(&env, "d20990101");
    assert_eq!(client.get_root(&batch_date), None);
}
