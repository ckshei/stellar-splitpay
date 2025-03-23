#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, token, String};

#[contract]
pub struct PaymentSplitter;

// Hardcoded addresses for payment splitting
const RECIPIENT_1: &str = "GDVBPBYEEDL7WLQ6GALFEGOX76PTXWQAIVMQIKJBKDU5SJ34KFI6NQ66";
const RECIPIENT_2: &str = "GBBPJQPG6ZOG6223UALTRV3HF7IJK3NKF7WA6VILPG67GDIGXIFKA633";
const RECIPIENT_3: &str = "GARTNJE76NYERBUWBSDP457OQ5HYCYNRERNNCMGNILNJY5W7T5OHWMPP";

// Default shares
const SHARE_1: u32 = 25; // 25%
const SHARE_2: u32 = 25; // 25%
const SHARE_3: u32 = 50; // 50%

#[contractimpl]
impl PaymentSplitter {
    // Split the contract's token balance between the three recipients
    pub fn split_balance(env: Env, token_id: Address) {
        // Get contract address
        let contract_address = env.current_contract_address();
        
        // Create Address objects from the hardcoded strings - fixed for SDK 22
        let recipient1 = Address::from_string(&String::from_str(&env, RECIPIENT_1));
        let recipient2 = Address::from_string(&String::from_str(&env, RECIPIENT_2));
        let recipient3 = Address::from_string(&String::from_str(&env, RECIPIENT_3));
        
        // Get the token client
        let token_client = token::Client::new(&env, &token_id);
        
        // Get contract's balance
        let balance = token_client.balance(&contract_address);
        
        // Calculate amounts based on shares
        let total_shares = SHARE_1 + SHARE_2 + SHARE_3;
        let amount1 = (balance * SHARE_1 as i128) / total_shares as i128;
        let amount2 = (balance * SHARE_2 as i128) / total_shares as i128;
        // Send remainder to third address to avoid rounding issues
        let amount3 = balance - amount1 - amount2;
        
        // Transfer tokens to recipients if amounts are positive
        if amount1 > 0 {
            token_client.transfer(&contract_address, &recipient1, &amount1);
        }
        
        if amount2 > 0 {
            token_client.transfer(&contract_address, &recipient2, &amount2);
        }
        
        if amount3 > 0 {
            token_client.transfer(&contract_address, &recipient3, &amount3);
        }
    }
}