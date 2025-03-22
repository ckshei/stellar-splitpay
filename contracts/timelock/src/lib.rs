#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Address, Env, Vec, Map,
    symbol_short, token, unwrap::UnwrapOptimized,
};

#[contracttype]
pub struct TimeLock {
    lock_id: u64,
    amount: i128,
    unlock_time: u64,
    max_amount: i128,
}

#[contracttype]
pub enum DataKey {
    TimeLock(Address, u64), // User address and lock ID
    UserLocks(Address),     // Maps a user to their lock IDs
    NextLockId,
    Admin,
}

fn get_token_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap_or_else(|| {
        panic!("admin not set")
    })
}

fn verify_admin(e: &Env, admin: &Address) {
    admin.require_auth();
    let stored_admin = get_token_admin(e);
    if *admin != stored_admin {
        panic!("not authorized by admin");
    }
}

fn get_time_lock(e: &Env, user: &Address, lock_id: u64) -> Option<TimeLock> {
    e.storage().instance().get(&DataKey::TimeLock(user.clone(), lock_id))
}

fn get_user_locks(e: &Env, user: &Address) -> Vec<u64> {
    e.storage().instance().get(&DataKey::UserLocks(user.clone())).unwrap_or_else(|| {
        Vec::new(e)
    })
}

fn get_next_lock_id(e: &Env) -> u64 {
    e.storage().instance().get(&DataKey::NextLockId).unwrap_or(0)
}

fn increment_lock_id(e: &Env) -> u64 {
    let current_id = get_next_lock_id(e);
    let new_id = current_id + 1;
    e.storage().instance().set(&DataKey::NextLockId, &new_id);
    new_id
}

#[contract]
pub struct TimelockContract;

#[contractimpl]
impl TimelockContract {
    pub fn init(e: Env, admin: Address) {
        e.storage().instance().set(&DataKey::Admin, &admin);
        e.storage().instance().set(&DataKey::NextLockId, &0u64);
    }

    pub fn create_lock(
        e: Env,
        user: Address,
        max_amount: i128,
        lock_period: u64,
    ) -> u64 {
        user.require_auth();

        if max_amount <= 0 {
            panic!("max_amount must be positive");
        }

        let current_time = e.ledger().timestamp();
        let unlock_time = current_time + lock_period;
        
        // Get a new lock ID
        let lock_id = increment_lock_id(&e);
        
        // Create a new timelock with zero initial amount
        let new_lock = TimeLock {
            lock_id,
            amount: 0,
            unlock_time,
            max_amount,
        };
        
        // Store the new lock
        e.storage().instance().set(&DataKey::TimeLock(user.clone(), lock_id), &new_lock);
        
        // Add the lock ID to the user's list of locks
        let mut user_locks = get_user_locks(&e, &user);
        user_locks.push_back(lock_id);
        e.storage().instance().set(&DataKey::UserLocks(user.clone()), &user_locks);
        
        lock_id
    }

    pub fn deposit(
        e: Env,
        token_id: Address,
        user: Address,
        lock_id: u64,
        amount: i128,
    ) {
        user.require_auth();

        if amount <= 0 {
            panic!("amount must be positive");
        }

        let mut timelock = get_time_lock(&e, &user, lock_id).unwrap_optimized();
        
        // Check if we're still within the maximum
        if timelock.amount + amount > timelock.max_amount {
            panic!("deposit would exceed maximum amount");
        }
        
        // Update the timelock with new amount
        timelock.amount += amount;
        e.storage().instance().set(&DataKey::TimeLock(user.clone(), lock_id), &timelock);

        // Transfer tokens from user to contract
        let client = token::Client::new(&e, &token_id);
        client.transfer(&user, &e.current_contract_address(), &amount);
    }

    pub fn withdraw(e: Env, token_id: Address, user: Address, lock_id: u64) -> i128 {
        user.require_auth();

        let timelock = get_time_lock(&e, &user, lock_id).unwrap_optimized();
        let current_time = e.ledger().timestamp();

        if current_time < timelock.unlock_time {
            panic!("tokens are still locked");
        }

        let amount = timelock.amount;
        
        // Set amount to zero but keep the lock structure for history
        let updated_lock = TimeLock {
            lock_id: timelock.lock_id,
            amount: 0,
            unlock_time: timelock.unlock_time,
            max_amount: timelock.max_amount,
        };
        
        e.storage().instance().set(&DataKey::TimeLock(user.clone(), lock_id), &updated_lock);

        // Transfer tokens from contract to user
        let client = token::Client::new(&e, &token_id);
        client.transfer(&e.current_contract_address(), &user, &amount);

        amount
    }

    pub fn get_timelock(e: Env, user: Address, lock_id: u64) -> Option<TimeLock> {
        get_time_lock(&e, &user, lock_id)
    }

    pub fn get_user_timelocks(e: Env, user: Address) -> Vec<TimeLock> {
        let lock_ids = get_user_locks(&e, &user);
        let mut locks = Vec::new(&e);
        
        for i in 0..lock_ids.len() {
            let lock_id = lock_ids.get(i).unwrap_optimized();
            if let Some(lock) = get_time_lock(&e, &user, lock_id) {
                locks.push_back(lock);
            }
        }
        
        locks
    }

    pub fn get_all_users_with_locks(e: Env, admin: Address) -> Vec<Address> {
        verify_admin(&e, &admin);
        
        // Similar to the previous version, this would require maintaining a separate list
        // of all users with locks. This is a placeholder.
        Vec::new(&e)
    }

    pub fn get_lock_status(e: Env, user: Address, lock_id: u64) -> (i128, u64, bool) {
        match get_time_lock(&e, &user, lock_id) {
            Some(timelock) => {
                let current_time = e.ledger().timestamp();
                let is_unlocked = current_time >= timelock.unlock_time;
                (timelock.amount, timelock.unlock_time, is_unlocked)
            }
            None => (0, 0, false),
        }
    }
}