mod models;
mod utils;

use crate::{models::RL2020, utils::AccountId};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::PanicOnDefault;
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen, PromiseIndex};

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    rls: LookupMap<String, (String, RL2020)>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            owner: AccountId::from("metadid.testnet"),
            rls: LookupMap::new(b"r"),
        }
    }

    /// register a new revocation list
    pub fn register_list(&mut self, id: String) {
        if id.trim().is_empty() {
            env::panic_str("ERR_INVALID_RL_LIST");
        }
        if self.rls.contains_key(&id) {
            env::panic_str("ERR_RL_EXISTS");
        }
        let rl = RL2020::new().unwrap_or_else(|e| env::panic_str(&e.message));

        let owner = env::predecessor_account_id().to_string();
        self.rls.insert(&id, &(owner, rl));
        env::log_str("Added a new revocation list");
    }

    pub fn get_encoded_list(&self, id: String) -> String {
        let rl = self
            .rls
            .get(&id)
            .unwrap_or_else(|| env::panic_str("ERR_RL_NOT_FOUND"));

        rl.to_string()
    }

    pub fn is_revoked(&self, id: String, idx: u64) -> bool {
        let rl = self
            .rls
            .get(&id)
            .unwrap_or_else(|| env::panic_str("ERR_RL_NOT_FOUND"));
        rl.get(idx).unwrap_or_else(|e| env::panic_str(&e.message))
    }

    fn check_permission((ref owner, _): &(String, RL2020)) {
        if env::predecessor_account_id() != self.owner {
            env::panic_str("ERR_NOT_AUTHORIZED");
        }
    }

    pub fn set_list(&mut self, id: String, hex_encoded_list: String) {
        let mut rl = self.rls.get(&id).unwrap_or_else(|| {
            env::panic_str("ERR_RL_NOT_FOUND");
        });
        Self::check_permission(&rl);
        let encoded_list = hex::decode(hex_encoded_list).unwrap_or_else(|e| {
            env::panic_str(&e.to_string());
        });
        rl.replace(encoded_list)
            .unwrap_or_else(|e| env::panic_str(&e.message));
        self.rls.insert(&id, &rl);
    }

    /// revoke a credential
    pub fn revoke(&mut self, id: String, idx: u64) {
        self.set(id, idx, true)
    }

    // reset a credential to not revoked
    pub fn reset(&mut self, id: String, idx: u64) {
        self.set(id, idx, false)
    }

    /// Update a revocation list with a list of ids to revoke and reset
    /// reset is a list of ids to reset to not revoked
    /// revoke is a list of ids to revoke
    /// reset take precedence over revoke
    pub fn update(&mut self, id: String, to_revoke: Vec<u64>, to_reset: Vec<u64>) {
        let mut rl = self
            .rls
            .get(&id)
            .unwrap_or_else(|| env::panic_str("ERR_RL_NOT_FOUND"));
       
        Self::check_permission(&rl);

        rl.set_many(to_revoke, to_reset)
            .unwrap_or_else(|e| env::panic_str(&e.message));

        self.rls.insert(&id, &rl);
        env::log_str("revocation list updated");
    }

    fn set(&mut self, id: String, idx: u64, revoked: bool) {
        let mut rl = self
            .rls
            .get(&id)
            .unwrap_or_else(|| env::panic_str("ERR_RL_NOT_FOUND"));
        let revoker = env::predecessor_account_id();
        if rl.creator.to_string() != revoker.to_string() {
            env::panic_str("ERR_UNAUTHORIZED");
        }

        match revoked {
            true => rl.set_many(vec![idx], vec![]),
            false => rl.set_many(vec![], vec![idx]),
        }
        .unwrap_or_else(|e| env::panic_str(&e.message));

        self.rls.insert(&id, &rl);
        env::log_str("revocation list element updated");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn add_revocation_list() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new();

        contract.add_list("example/rl/1".to_string());
        let result = contract
            .get_encoded_list("example/rl/1".to_string())
            .to_string();

        assert_eq!(
            &result,
            "eJztwDEBAAAAwqD1T20MHygAAAAAAAAAAAAAAAAAAADgbUAAAAE="
        );
    }

    #[test]
    fn test_revoke_reset() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());

        testing_env!(context.build());

        let mut contract = Contract::new();

        let id = "example/rl/1";

        contract.add_list(id.to_string());

        let idx = 3214;
        contract.revoke(id.to_string(), idx);
        let is_revoked = contract.is_revoked(id.to_string(), idx);
        assert_eq!(is_revoked, true);

        let result = contract.get_encoded_list("example/rl/1".to_string());

        assert_eq!(
            &result,
            "eJztwAENAAAIwKBHs38qa7gJxSkTAAAAAAAAAAAAAAAAAAAADy3coQBB"
        );

        contract.reset(id.to_string(), idx);
        let is_revoked = contract.is_revoked(id.to_string(), idx);
        assert_eq!(is_revoked, false);

        let result = contract.get_encoded_list("example/rl/1".to_string());

        assert_eq!(
            &result,
            "eJztwDEBAAAAwqD1T20MHygAAAAAAAAAAAAAAAAAAADgbUAAAAE="
        );
    }
}
