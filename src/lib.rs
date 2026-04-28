#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol, Map, log,
};

// ─────────────────────────────────────────────
//  Storage Keys
// ─────────────────────────────────────────────

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
    TreasuryBalance(Address),
    Admin,
    TreasuryContract,
}

// ─────────────────────────────────────────────
//  Events
// ─────────────────────────────────────────────

const DEPOSIT_EVENT: Symbol = symbol_short!("deposit");
const CHARGE_EVENT: Symbol  = symbol_short!("charge");

// ─────────────────────────────────────────────
//  Contract
// ─────────────────────────────────────────────

#[contract]
pub struct MicropayContract;

#[contractimpl]
impl MicropayContract {

    // ── Initialise ──────────────────────────────────────────────────────────

    /// Call once at deployment. Sets the admin (backend signer) and the
    /// treasury contract address that is allowed to fund accounts.
    pub fn initialize(
        env: Env,
        admin: Address,
        treasury_contract: Address,
    ) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TreasuryContract, &treasury_contract);
    }

    // ── Deposits ────────────────────────────────────────────────────────────

    /// Called by the DAO Treasury contract to fund a user account.
    /// Only the registered treasury contract address may call this.
    pub fn deposit_from_treasury(
        env: Env,
        user: Address,
        amount: i128,
    ) {
        assert!(amount > 0, "amount must be positive");

        let treasury: Address = env
            .storage()
            .instance()
            .get(&DataKey::TreasuryContract)
            .expect("not initialized");

        treasury.require_auth();

        // Track treasury-originated funds separately for transparency
        let prev_treasury: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::TreasuryBalance(user.clone()))
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::TreasuryBalance(user.clone()), &(prev_treasury + amount));

        // Also add to spendable balance
        let prev: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(user.clone()))
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::Balance(user.clone()), &(prev + amount));

        env.events().publish(
            (DEPOSIT_EVENT, symbol_short!("treasury")),
            (user, amount),
        );
    }

    /// Called by a user to top-up their own balance.
    pub fn deposit_from_user(
        env: Env,
        user: Address,
        amount: i128,
    ) {
        assert!(amount > 0, "amount must be positive");
        user.require_auth();

        let prev: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(user.clone()))
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::Balance(user.clone()), &(prev + amount));

        env.events().publish(
            (DEPOSIT_EVENT, symbol_short!("user")),
            (user, amount),
        );
    }

    // ── Charging ────────────────────────────────────────────────────────────

    /// Deducts `amount` from `user`'s balance. Only the admin (backend) may call.
    pub fn charge(
        env: Env,
        user: Address,
        amount: i128,
    ) {
        assert!(amount > 0, "amount must be positive");

        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        admin.require_auth();

        let balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(user.clone()))
            .unwrap_or(0);

        if balance < amount {
            panic!("insufficient balance: overdraft prevented");
        }

        env.storage()
            .persistent()
            .set(&DataKey::Balance(user.clone()), &(balance - amount));

        env.events().publish(
            (CHARGE_EVENT, symbol_short!("api")),
            (user, amount),
        );
    }

    // ── Queries ─────────────────────────────────────────────────────────────

    /// Returns total spendable balance for `user`.
    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(user))
            .unwrap_or(0)
    }

    /// Returns how much of the user's balance originated from the treasury.
    pub fn get_treasury_balance(env: Env, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::TreasuryBalance(user))
            .unwrap_or(0)
    }

    // ── Admin helpers ────────────────────────────────────────────────────────

    /// Update treasury contract address (admin only).
    pub fn update_treasury(
        env: Env,
        new_treasury: Address,
    ) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("not initialized");
        admin.require_auth();
        env.storage().instance().set(&DataKey::TreasuryContract, &new_treasury);
    }
}
