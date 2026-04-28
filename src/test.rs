#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup() -> (Env, MicropayContractClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, MicropayContract);
    let client = MicropayContractClient::new(&env, &contract_id);

    let admin    = Address::generate(&env);
    let treasury = Address::generate(&env);
    let user     = Address::generate(&env);

    client.initialize(&admin, &treasury);

    (env, client, admin, treasury, user)
}

#[test]
fn test_deposit_from_treasury_and_get_balance() {
    let (_env, client, _admin, treasury, user) = setup();

    client.deposit_from_treasury(&user, &1_000_000);

    assert_eq!(client.get_balance(&user), 1_000_000);
    assert_eq!(client.get_treasury_balance(&user), 1_000_000);
}

#[test]
fn test_deposit_from_user() {
    let (_env, client, _admin, _treasury, user) = setup();

    client.deposit_from_user(&user, &500_000);

    assert_eq!(client.get_balance(&user), 500_000);
    assert_eq!(client.get_treasury_balance(&user), 0); // not treasury funded
}

#[test]
fn test_charge_deducts_balance() {
    let (_env, client, _admin, _treasury, user) = setup();

    client.deposit_from_user(&user, &1_000_000);
    client.charge(&user, &100_000);

    assert_eq!(client.get_balance(&user), 900_000);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn test_charge_prevents_overdraft() {
    let (_env, client, _admin, _treasury, user) = setup();

    client.deposit_from_user(&user, &50_000);
    client.charge(&user, &100_000); // should panic
}

#[test]
fn test_mixed_funding_sources() {
    let (_env, client, _admin, _treasury, user) = setup();

    client.deposit_from_treasury(&user, &400_000);
    client.deposit_from_user(&user, &600_000);

    assert_eq!(client.get_balance(&user), 1_000_000);
    assert_eq!(client.get_treasury_balance(&user), 400_000);
}

#[test]
fn test_charge_partial_many_times() {
    let (_env, client, _admin, _treasury, user) = setup();

    client.deposit_from_user(&user, &1_000_000);

    for _ in 0..10 {
        client.charge(&user, &50_000);
    }

    assert_eq!(client.get_balance(&user), 500_000);
}
