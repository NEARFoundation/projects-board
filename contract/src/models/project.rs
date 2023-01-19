use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};
use uuid::Builder;

use crate::{Contract, ContractExt};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Project {
    id: AccountId,
    name: String,
    description: String,
    link: String,
    github: String,
}

#[near_bindgen]
impl Contract {
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

    pub fn create_project(&mut self, create_project: Project) -> Option<Project> {
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

    pub fn update_project(&mut self, project_id: String, update_project: Project) {
        self.projects
            .entry(project_id)
            .and_modify(|project| *project = update_project.into())
            .or_insert_with(|| env::panic_str("No project with provided ID found!"));
    }

    pub fn remove_project(&mut self, project_id: String) -> Option<Project> {
        self.projects.remove(&project_id)
    }
}
