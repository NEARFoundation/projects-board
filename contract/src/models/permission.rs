use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::{Contract, ContractExt};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PermissionType {
    name: String,
    description: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Permission {
    id: String,
    description: String,
    contributor_id: AccountId,
    project_id: AccountId,
    type_id: String,
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PermissionView {
    id: String,
    description: String,
    contributor_id: AccountId,
    project_id: AccountId,
    permission_type: PermissionType,
}

#[near_bindgen]
impl Contract {
    pub fn get_permission(&self, permission_id: String) -> Option<PermissionView> {
        let permission = self.permissions.get(&permission_id)?.clone();
        let permission_type = self.permission_types.get(&permission.type_id)?.clone();

        Some(PermissionView {
            id: permission.id,
            description: permission.description,
            project_id: permission.project_id,
            contributor_id: permission.contributor_id,
            permission_type,
        })
    }

    pub fn create_permission(&mut self, permission: Permission) -> Option<PermissionView> {
        let permission_type = self.permission_types.get(&permission.type_id)?.clone();

        let id = match String::from_utf8(near_sdk::env::random_seed()) {
            Ok(id) => id,
            _ => {
                return None;
            }
        };

        let mut permission = permission;

        permission.id = id.clone();

        let permission = self.permissions.insert(id.clone(), permission)?;

        Some(PermissionView {
            id,
            description: permission.description,
            project_id: permission.project_id,
            contributor_id: permission.contributor_id,
            permission_type,
        })
    }

    pub fn update_permission(
        &mut self,
        permission_id: String,
        permission: Permission,
    ) -> Option<PermissionView> {
        if !self.permissions.contains_key(&permission_id) {
            return None;
        }

        let permission_type = self.permission_types.get(&permission.type_id)?.clone();

        let permission = self.permissions.insert(permission_id, permission)?;

        Some(PermissionView {
            id: permission.id,
            description: permission.description,
            project_id: permission.project_id,
            contributor_id: permission.contributor_id,
            permission_type,
        })
    }

    pub fn remove_permission(&mut self, permission_id: String) -> Option<PermissionView> {
        let permission = self.permissions.remove(&permission_id)?;
        let permission_type = self.permission_types.get(&permission.type_id)?.clone();

        Some(PermissionView {
            id: permission.id,
            description: permission.description,
            project_id: permission.project_id,
            contributor_id: permission.contributor_id,
            permission_type,
        })
    }

    pub fn get_permission_type(&self, permission_type: String) -> Option<PermissionType> {
        self.permission_types.get(&permission_type).cloned()
    }

    pub fn get_permission_types(&self) -> Vec<PermissionType> {
        self.permission_types
            .into_iter()
            .map(|(_, permission_type)| permission_type)
            .cloned()
            .collect()
    }

    pub fn create_permission_type(
        &mut self,
        permission_type: PermissionType,
    ) -> Option<PermissionType> {
        if !self.permission_types.contains_key(&permission_type.name) {
            self.permission_types
                .insert(permission_type.name.clone(), permission_type)
        } else {
            None
        }
    }

    pub fn update_permission_type(
        &mut self,
        permission_type: PermissionType,
    ) -> Option<PermissionType> {
        if self.permission_types.contains_key(&permission_type.name) {
            self.permission_types
                .insert(permission_type.name.clone(), permission_type)
        } else {
            None
        }
    }

    pub fn remove_permission_type(&mut self, permission_type: String) -> Option<PermissionType> {
        self.permission_types.remove(&permission_type)
    }
}
