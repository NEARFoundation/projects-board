use std::collections::HashSet;
use std::io::Read;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::store::{UnorderedMap, UnorderedSet, Vector};
use near_sdk::{env, log, near_bindgen, AccountId};
use uuid::{Builder, Uuid};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Project {
    name: String,
    details: String,
    verified: bool,
    contributors: HashSet<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CreateProject {
    name: String,
    details: String,
}

impl From<CreateProject> for Project {
    fn from(value: CreateProject) -> Self {
        Self {
            name: value.name,
            details: value.details,
            verified: false,
            contributors: HashSet::new(),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contributor {
    name: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Contribution {
    Code(String),
    Investment(String),
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Connection {
    RequestedByContributor,
    RequestedByProject,
    Ongoing {
        contrbutions: Vector<Contribution>,
        started_at: u64,
    },
    Finished {
        contributions: Vector<Contribution>,
        finished_at: u64,
    },
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    projects: UnorderedMap<String, Project>,
    contributors: UnorderedMap<String, Contributor>,
    connections: UnorderedMap<(String, String), Connection>,
    moderators: UnorderedSet<AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            projects: UnorderedMap::new(b"p"),
            contributors: UnorderedMap::new(b"c"),
            connections: UnorderedMap::new(b"n"),
            moderators: UnorderedSet::new(b"m"),
        }
    }
}

#[near_bindgen]
impl Contract {
    /*
     * PROJECTS SECTION
     */

    pub fn get_project(&self, project_id: String) -> Option<Project> {
        self.projects.get(&project_id).cloned()
    }

    pub fn get_projects(&self, limit: Option<usize>, from: Option<usize>) -> Vec<Project> {
        self.projects
            .iter()
            .skip(from.unwrap_or_default())
            .take(limit.unwrap_or_else(|| 20))
            .map(|(_, project)| project)
            .cloned()
            .collect()
    }

    pub fn create_project(&mut self, create_project: CreateProject) -> Option<Project> {
        let mut random_bytes_array: [u8; 16] = [0; 16];

        for (index, byte) in env::random_seed().iter().enumerate().take(16) {
            random_bytes_array[index] = byte.clone();
        }

        self.projects.insert(
            Builder::from_bytes(random_bytes_array)
                .as_uuid()
                .to_string(),
            create_project.into(),
        )
    }

    pub fn update_project(&mut self, project_id: String, update_project: CreateProject) {
        self.projects
            .entry(project_id)
            .and_modify(|project| *project = update_project.into())
            .or_insert_with(|| env::panic_str("No project with provided ID found!"));
    }

    pub fn remove_project(&mut self, project_id: String) -> Option<Project> {
        self.projects.remove(&project_id)
    }

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
