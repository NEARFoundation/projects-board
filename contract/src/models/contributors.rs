use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, require,
    store::UnorderedSet,
    AccountId,
};

use crate::{Contract, ContractExt};

use super::contribution::ContributionType;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contributor {
    id: AccountId,
    contribution_types_currently_offered: UnorderedSet<String>,
}

#[near_bindgen]
impl Contract {
    pub fn add_contributor_contribution_types(
        &mut self,
        contribution_types: Vec<String>,
    ) -> Option<Vec<ContributionType>> {
        require!(
            contribution_types
                .iter()
                .all(|name| self.contribution_types.contains_key(name)),
            "Some of the provided contribution types do not exist!"
        );

        let contributor = self
            .contributors
            .entry(env::signer_account_id())
            .and_modify(|contributor| {
                contributor
                    .contribution_types_currently_offered
                    .extend(contribution_types.clone());
            })
            .or_insert(Contributor {
                id: env::signer_account_id(),
                contribution_types_currently_offered: {
                    let mut set = UnorderedSet::new(env::signer_account_id().as_bytes());
                    set.extend(contribution_types);
                    set
                },
            });

        Some(
            contributor
                .contribution_types_currently_offered
                .iter()
                .filter_map(|name| self.contribution_types.get(name))
                .cloned()
                .collect(),
        )
    }

    pub fn remove_contributor_contribution_types(
        &mut self,
        contribution_types: Vec<String>,
    ) -> Option<Vec<ContributionType>> {
        let contributor = self
            .contributors
            .entry(env::signer_account_id())
            .and_modify(|contributor| {
                contribution_types.iter().for_each(|name| {
                    contributor
                        .contribution_types_currently_offered
                        .remove(name);
                })
            })
            .or_insert(Contributor {
                id: env::signer_account_id(),
                contribution_types_currently_offered: UnorderedSet::new(
                    env::signer_account_id().as_bytes(),
                ),
            });

        Some(
            contributor
                .contribution_types_currently_offered
                .iter()
                .filter_map(|name| self.contribution_types.get(name))
                .cloned()
                .collect(),
        )
    }

    pub fn get_contributor_contribution_types(
        &self,
        contributor_id: AccountId,
    ) -> Option<Vec<ContributionType>> {
        Some(
            self.contributors
                .get(&contributor_id)?
                .contribution_types_currently_offered
                .iter()
                .filter_map(|name| self.contribution_types.get(name))
                .cloned()
                .collect(),
        )
    }
}
