#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Initialized,
    Backend,
    Treasury,
    TreasuryFunded(Address),
    UserFunded(Address),
}

#[contract]
pub struct MicropayContract;

fn assert_initialized(env: &Env) {
    if !env
        .storage()
        .instance()
        .get::<DataKey, bool>(&DataKey::Initialized)
        .unwrap_or(false)
    {
        panic!("contract_not_initialized");
    }
}

fn get_amount(env: &Env, key: DataKey) -> i128 {
    env.storage().persistent().get::<DataKey, i128>(&key).unwrap_or(0)
}

fn add_amount(env: &Env, key: DataKey, amount: i128) {
    let current = get_amount(env, key.clone());
    let updated = current
        .checked_add(amount)
        .unwrap_or_else(|| panic!("balance_overflow"));
    env.storage().persistent().set(&key, &updated);
}

fn subtract_amount(env: &Env, key: DataKey, amount: i128) {
    let current = get_amount(env, key.clone());
    if current < amount {
        panic!("insufficient_balance");
    }
    env.storage().persistent().set(&key, &(current - amount));
}

fn emit_deposit(env: &Env, funding_source: Symbol, user: Address, amount: i128) {
    env.events()
        .publish((symbol_short!("deposit"), funding_source, user), amount);
}

fn emit_charge(env: &Env, user: Address, amount: i128) {
    env.events().publish((symbol_short!("charge"), user), amount);
}

#[contractimpl]
impl MicropayContract {
    pub fn init(env: Env, backend: Address, treasury: Address) {
        if env
            .storage()
            .instance()
            .get::<DataKey, bool>(&DataKey::Initialized)
            .unwrap_or(false)
        {
            panic!("already_initialized");
        }

        backend.require_auth();
        treasury.require_auth();

        env.storage().instance().set(&DataKey::Backend, &backend);
        env.storage().instance().set(&DataKey::Treasury, &treasury);
        env.storage().instance().set(&DataKey::Initialized, &true);
    }

    pub fn deposit_from_treasury(env: Env, user: Address, amount: i128) {
        assert_initialized(&env);
        if amount <= 0 {
            panic!("invalid_amount");
        }

        let treasury = env
            .storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Treasury)
            .unwrap_or_else(|| panic!("treasury_not_set"));

        treasury.require_auth();
        add_amount(&env, DataKey::TreasuryFunded(user.clone()), amount);
        emit_deposit(&env, symbol_short!("treasury"), user, amount);
    }

    pub fn deposit_from_user(env: Env, user: Address, amount: i128) {
        assert_initialized(&env);
        if amount <= 0 {
            panic!("invalid_amount");
        }

        user.require_auth();
        add_amount(&env, DataKey::UserFunded(user.clone()), amount);
        emit_deposit(&env, symbol_short!("user"), user, amount);
    }

    pub fn charge(env: Env, user: Address, amount: i128) {
        assert_initialized(&env);
        if amount <= 0 {
            panic!("invalid_amount");
        }

        let backend = env
            .storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Backend)
            .unwrap_or_else(|| panic!("backend_not_set"));
        backend.require_auth();

        // Consume treasury-funded balance first so DAO allocations are auditable.
        let treasury_key = DataKey::TreasuryFunded(user.clone());
        let user_key = DataKey::UserFunded(user.clone());
        let treasury_balance = get_amount(&env, treasury_key.clone());
        let user_balance = get_amount(&env, user_key.clone());
        if treasury_balance + user_balance < amount {
            panic!("insufficient_balance");
        }

        if treasury_balance >= amount {
            subtract_amount(&env, treasury_key, amount);
        } else {
            let remainder = amount - treasury_balance;
            if treasury_balance > 0 {
                env.storage().persistent().set(&treasury_key, &0_i128);
            }
            subtract_amount(&env, user_key, remainder);
        }

        emit_charge(&env, user, amount);
    }

    pub fn get_balance(env: Env, user: Address) -> i128 {
        assert_initialized(&env);
        get_amount(&env, DataKey::TreasuryFunded(user.clone()))
            + get_amount(&env, DataKey::UserFunded(user))
    }

    pub fn get_balance_breakdown(env: Env, user: Address) -> (i128, i128) {
        assert_initialized(&env);
        (
            get_amount(&env, DataKey::TreasuryFunded(user.clone())),
            get_amount(&env, DataKey::UserFunded(user)),
        )
    }
}
