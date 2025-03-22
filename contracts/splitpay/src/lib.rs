#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, token, Symbol};

#[contract]
pub struct PaymentSplitter;

#[contractimpl]
impl PaymentSplitter {
    // Initialize with three recipient addresses and their respective shares
    pub fn initialize(
        env: Env,
        recipient1: Address,
        recipient2: Address,
        recipient3: Address,
        share1: u32,
        share2: u32,
        share3: u32,
    ) {
        // Verify the contract hasn't been initialized already
        if env.storage().instance().has(&Symbol::short("init")) {
            panic!("Contract already initialized");
        }
        
        // Store recipients
        env.storage().instance().set(&Symbol::short("r1"), &recipient1);
        env.storage().instance().set(&Symbol::short("r2"), &recipient2);
        env.storage().instance().set(&Symbol::short("r3"), &recipient3);
        
        // Store shares
        env.storage().instance().set(&Symbol::short("s1"), &share1);
        env.storage().instance().set(&Symbol::short("s2"), &share2);
        env.storage().instance().set(&Symbol::short("s3"), &share3);
        
        // Mark as initialized
        env.storage().instance().set(&Symbol::short("init"), &true);
    }
    
    // Split a payment between the three recipients
    pub fn split_payment(env: Env, sender: Address, token_id: Address, amount: i128) {
        // Require authorization from sender
        sender.require_auth();
        
        // Get recipients and their shares
        let recipient1: Address = env.storage().instance().get(&Symbol::short("r1")).unwrap();
        let recipient2: Address = env.storage().instance().get(&Symbol::short("r2")).unwrap();
        let recipient3: Address = env.storage().instance().get(&Symbol::short("r3")).unwrap();
        
        let share1: u32 = env.storage().instance().get(&Symbol::short("s1")).unwrap();
        let share2: u32 = env.storage().instance().get(&Symbol::short("s2")).unwrap();
        let share3: u32 = env.storage().instance().get(&Symbol::short("s3")).unwrap();
        
        // Calculate total shares
        let total_shares = share1 + share2 + share3;
        
        // Calculate individual amounts
        let amount1 = (amount * share1 as i128) / total_shares as i128;
        let amount2 = (amount * share2 as i128) / total_shares as i128;
        // Calculate amount3 as remainder to avoid rounding errors
        let amount3 = amount - amount1 - amount2;
        
        // Create token client
        let token_client = token::Client::new(&env, &token_id);
        
        // Transfer from sender to recipients
        token_client.transfer(&sender, &recipient1, &amount1);
        token_client.transfer(&sender, &recipient2, &amount2);
        token_client.transfer(&sender, &recipient3, &amount3);
    }
}