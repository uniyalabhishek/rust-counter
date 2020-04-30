//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! [get it's current value][get_num] or [reset].
//!
//! [increment]: struct.Counter.html#method.increment
//! [decrement]: struct.Counter.html#method.decrement
//! [get_num]: struct.Counter.html#method.get_num
//! [reset]: struct.Counter.html#method.reset
use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    user_counters: HashMap<String, i8>,
}

#[near_bindgen]
impl Counter {
    /// Init attribute used for instantiation.
    #[init]
    pub fn new() -> Self {
        // useful snippet to copy/paste, making sure state isn't already initialized
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        // notice we've chosen to use an implicit "return" here
        Self {
            user_counters: HashMap::new(),
        }
    }

    /// Returns 8-bit signed integer representing the number for the account argument.
    ///
    /// Note, the parameter is &self (without being mutable) meaning it doesn't modify state.
    /// In the frontend (/src/main.js) this is added to the "viewMethods" array
    /// using near-shell we can call this by:
    ///
    /// ```bash
    /// near view counter.YOU.testnet get_num '{"account": "donation.YOU.testnet"}'
    /// ```
    pub fn get_num(&self, account: String) -> i8 {
        // call our first private function
        // try removing the .clone() below and note the error. this may happen from time to time
        // (learn more about Rust ownership later: https://doc.rust-lang.org/nomicon/ownership.html)
        let caller_num = self.get_num_from_signer(account.clone());

        // here's a way to format multiple variables in order to log them
        let log_message = format!("{}'s number: {}", account, caller_num);
        env::log(log_message.as_bytes());
        // notice we've chosen to use an implicit "return" here
        caller_num
    }

    // our first private functions
    fn get_num_from_signer(&self, account: String) -> i8 {
        // notice we've chosen to use an implicit "return" here
        self.user_counters.get(&account).cloned().unwrap_or(0)
    }

    /// Increment the counter *per account* that calls it.
    ///
    /// Note, the parameter is "&mut self" as this function modifies state.
    /// In the frontend (/src/main.js) this is added to the "changeMethods" array
    /// using near-shell we can call this by:
    ///
    /// ```bash
    /// near call counter.YOU.testnet increment --accountId donation.YOU.testnet
    /// ```
    pub fn increment(&mut self) {
        // note: adding one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        let caller = env::signer_account_id();
        let current_val = self.user_counters.get(&caller).cloned().unwrap_or(0);
        self.user_counters.insert(caller.clone(), current_val + 1);

        // this will panic if it's not added (but we know it's there)
        let counter_val = self.user_counters[&caller];

        let log_message = format!("Incremented to {}", counter_val);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    /// Decrement (subtract from) the counter *per account* that calls it.
    ///
    /// In (/src/main.js) this is also added to the "changeMethods" array
    /// using near-shell we can call this by:
    ///
    /// ```bash
    /// near call counter.YOU.testnet decrement --accountId donation.YOU.testnet
    /// ```
    pub fn decrement(&mut self) {
        // note: subtracting one like this is an easy way to accidentally overflow
        // real smart contracts will want to have safety checks
        let caller = env::signer_account_id();
        // we'll use a slightly different approach to illustrate dereferencing (the "*")
        // see https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value
        let count = self.user_counters.entry(caller).or_insert(0);
        *count -= 1;

        let log_message = format!("Decreased number to {}", count);
        env::log(log_message.as_bytes());
        after_counter_change();
    }

    /// Reset to zero.
    pub fn reset(&mut self) {
        let caller = env::signer_account_id();
        self.user_counters.insert(caller, 0);
        // Another way to log on NEAR is to cast a string into bytes, hence "b" below:
        env::log(b"Reset counter to zero");
    }
}

// unlike the struct's functions above, this function cannot use attributes #[derive(…)] or #[near_bindgen]
// any attempts will throw helpful warnings upon 'cargo build'
// while this function cannot be invoked directly on the blockchain, it can be called from an invoked function
fn after_counter_change() {
    // show helpful warning that i8 (8-bit signed integer) will overflow above 127 or below -128
    env::log(b"Make sure you don't overflow, my friend.");
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test -- --nocapture
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool, signer: String) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: signer,
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    // unlike other frameworks, the function names don't need to be special or have "test" in it
    #[test]
    fn increment() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false, "robert.testnet".to_string());
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter::new();
        contract.increment();
        // we can do println! in tests, but reminder to use env::log outside of tests
        println!("Value after increment: {}", contract.get_num("robert.testnet".to_string()));
        // confirm that we received 1 when calling get_num
        assert_eq!(1, contract.get_num("robert.testnet".to_string()));
    }

    #[test]
    fn decrement() {
        let context = get_context(vec![], false, "robert.testnet".to_string());
        testing_env!(context);
        let mut contract = Counter::new();
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num("robert.testnet".to_string()));
        // confirm that we received -1 when calling get_num
        assert_eq!(-1, contract.get_num("robert.testnet".to_string()));
    }

    #[test]
    fn increment_and_reset() {
        let context = get_context(vec![], false, "robert.testnet".to_string());
        testing_env!(context);
        let mut contract = Counter::new();
        contract.increment();
        contract.reset();
        println!("Value after reset: {}", contract.get_num("robert.testnet".to_string()));
        // confirm that we received 0 after reset
        assert_eq!(0, contract.get_num("robert.testnet".to_string()));
    }

    #[test]
    fn use_two_accounts_and_verify() {
        let context_robert = get_context(vec![], false, "robert.testnet".to_string());
        let context_alice = get_context(vec![], false, "alice.testnet".to_string());
        // increment twice on robert's account
        testing_env!(context_robert);
        let mut contract = Counter::new();
        contract.increment();
        contract.increment();
        // decrement once on alice's account
        testing_env!(context_alice);
        contract.decrement();
        // confirm values per account
        println!("Value from robert.testnet's account: {}", contract.get_num("robert.testnet".to_string()));
        println!("Value from alice.testnet's account: {}", contract.get_num("alice.testnet".to_string()));
        assert_eq!(2, contract.get_num("robert.testnet".to_string()));
        assert_eq!(-1, contract.get_num("alice.testnet".to_string()));
    }
}
