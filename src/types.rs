use crate::*;

pub type TaskId = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum UserType {
    Requester {
        total_transfered: Balance,
        current_requests: u16,
    },
    Worker {
        total_received: Balance,
        current_applies: u16,
    },
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Task {
    pub owner: ValidAccountId,
    pub title: String,
    pub description: String,
    pub max_participants: u16,
    pub price: Balance,
    pub proposals: UnorderedMap<ValidAccountId, Proposal>,
    pub created_at: Timestamp,
    pub available_until: Timestamp,
    pub category_id: CategoryId,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
    pub account_id: ValidAccountId,
    pub bio: String,
    pub user_type: UserType,
    pub current_jobs: UnorderedSet<TaskId>,
    pub completed_jobs: UnorderedSet<TaskId>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub account_id: ValidAccountId,
    pub proof_of_work: String, //prefer an url like github repo or figma design files, etc
    pub is_approved: bool,
    pub is_rejected: bool,
}
