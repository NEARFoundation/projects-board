use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::{UnorderedMap, UnorderedSet, Vector};
use near_sdk::{log, near_bindgen, AccountId};

mod models;

use models::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    projects: UnorderedMap<String, project::Project>,
    permisssions: UnorderedMap<String, permission::Permission>,
    contributors: UnorderedSet<AccountId>,
    contributions: UnorderedMap<String, Vector<contribution::Contribution>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            projects: UnorderedMap::new(b"p"),
            permisssions: UnorderedMap::new(b"a"),
            contributors: UnorderedSet::new(b"c"),
            contributions: UnorderedMap::new(b"s"),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_greeting(&self) -> String {
        "".to_string()
    }

    pub fn set_greeting(&mut self, message: String) {
        log!("Saving greeting {}", message);
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello".to_string());
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy".to_string());
    }
}
