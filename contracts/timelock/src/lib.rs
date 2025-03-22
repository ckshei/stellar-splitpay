//! For simplicity, the contract only supports invoker-based auth.
#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    Balance,
    Initializer,          // Store the address that initialized this contract
    DeployedContracts,    // Map from address to list of contracts they initialized
}

#[derive(Clone)]
#[contracttype]
pub enum TimeBoundKind {
    Before,
    After,
}

#[derive(Clone)]
#[contracttype]
pub struct TimeBound {
    pub kind: TimeBoundKind,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct ClaimableBalance {
    pub token: Address,
    pub amount: i128,
    pub claimants: Vec<Address>,
    pub time_bound: TimeBound,
}

#[contract]
pub struct ClaimableBalanceContract;

// The 'timelock' part: check that provided timestamp is before/after
// the current ledger timestamp.
fn check_time_bound(env: &Env, time_bound: &TimeBound) -> bool {
    let ledger_timestamp = env.ledger().timestamp();

    match time_bound.kind {
        TimeBoundKind::Before => ledger_timestamp <= time_bound.timestamp,
        TimeBoundKind::After => ledger_timestamp >= time_bound.timestamp,
    }
}

#[contractimpl]
impl ClaimableBalanceContract {
    pub fn deposit(
        env: Env,
        from: Address,
        token: Address,
        amount: i128,
        claimants: Vec<Address>,
        time_bound: TimeBound,
    ) {
        if claimants.len() > 10 {
            panic!("too many claimants");
        }
        if is_initialized(&env) {
            panic!("contract has been already initialized");
        }
        // Make sure `from` address authorized the deposit call with all the
        // arguments.
        from.require_auth();

        // Transfer token from `from` to this contract address.
        token::Client::new(&env, &token).transfer(&from, &env.current_contract_address(), &amount);
        
        // Store all the necessary info to allow one of the claimants to claim it.
        env.storage().instance().set(
            &DataKey::Balance,
            &ClaimableBalance {
                token,
                amount,
                time_bound,
                claimants,
            },
        );
        
        // Store the initializer address
        env.storage().instance().set(&DataKey::Initializer, &from);
        
        // Get current list of contracts deployed by this address or create new empty list
        let mut contracts: Vec<Address> = env.storage()
            .persistent()
            .get(&(DataKey::DeployedContracts, from.clone()))
            .unwrap_or_else(|| Vec::new(&env));
            
        // Add this contract to the list
        contracts.push_back(env.current_contract_address());
        
        // Update the persistent storage with the new list
        env.storage()
            .persistent()
            .set(&(DataKey::DeployedContracts, from), &contracts);
            
        // Mark contract as initialized to prevent double-usage.
        env.storage().instance().set(&DataKey::Init, &());
    }

    pub fn claim(env: Env, claimant: Address) {
        // Make sure claimant has authorized this call, which ensures their
        // identity.
        claimant.require_auth();
        // Just get the balance - if it's been claimed, this will simply panic
        // and terminate the contract execution.
        let claimable_balance: ClaimableBalance =
            env.storage().instance().get(&DataKey::Balance).unwrap();

        if !check_time_bound(&env, &claimable_balance.time_bound) {
            panic!("time predicate is not fulfilled");
        }

        let claimants = &claimable_balance.claimants;
        if !claimants.contains(&claimant) {
            panic!("claimant is not allowed to claim this balance");
        }

        // Transfer the stored amount of token to claimant after passing
        // all the checks.
        token::Client::new(&env, &claimable_balance.token).transfer(
            &env.current_contract_address(),
            &claimant,
            &claimable_balance.amount,
        );
        // Remove the balance entry to prevent any further claims.
        env.storage().instance().remove(&DataKey::Balance);
    }
    
    // New method to get all contracts initialized by a specific address
    pub fn get_contracts_by_initializer(env: Env, initializer: Address) -> Vec<Address> {
        env.storage()
            .persistent()
            .get(&(DataKey::DeployedContracts, initializer))
            .unwrap_or_else(|| Vec::new(&env))
    }
    
    // Optional: Get the initializer of the current contract
    pub fn get_initializer(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Initializer).unwrap()
    }
}

fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Init)
}

mod test;