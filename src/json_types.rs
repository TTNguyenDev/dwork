use crate::*;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedTask {
    pub owner: ValidAccountId,
    pub title: String,
    pub description: String,
    pub max_participants: u16,
    pub price: WrappedBalance,
    pub proposals: Vec<WrappedProposal>,
    pub created_at: WrappedTimestamp,
    pub available_until: WrappedTimestamp,
    pub category_id: CategoryId,
}

impl From<Task> for WrappedTask {
    fn from(task: Task) -> Self {
        let wrapped_proposal: Vec<WrappedProposal> = task
            .proposals
            .iter()
            .map(|(_k, item)| WrappedProposal::from(item))
            .collect();

        WrappedTask {
            owner: task.owner,
            title: task.title,
            description: task.description,
            max_participants: task.max_participants,
            price: WrappedBalance::from(task.price),
            proposals: wrapped_proposal,
            created_at: WrappedTimestamp::from(task.created_at),
            available_until: WrappedTimestamp::from(task.available_until),
            category_id: task.category_id,
        }
    }
}

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedUser {
    pub account_id: ValidAccountId,
    pub bio: String,
    pub user_type: UserType,
    pub completed_jobs: Vec<TaskId>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum WrappedUserType {
    Requester {
        total_transfered: WrappedBalance,
        current_requests: u16,
    },
    Worker {
        total_received: WrappedBalance,
        current_applies: u16,
    },
}

impl From<UserType> for WrappedUserType {
    fn from(user_type: UserType) -> Self {
        match user_type {
            UserType::Requester {
                total_transfered,
                current_requests,
            } => WrappedUserType::Requester {
                total_transfered: WrappedBalance::from(total_transfered),
                current_requests,
            },
            UserType::Worker {
                total_received,
                current_applies,
            } => WrappedUserType::Worker {
                total_received: WrappedBalance::from(total_received),
                current_applies,
            },
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedProposal {
    pub account_id: ValidAccountId,
    pub proof_of_work: String, //prefer an url like github repo or figma design files, etc
    pub is_approved: bool,
    pub is_reject: bool,
}

impl From<Proposal> for WrappedProposal {
    fn from(proposal: Proposal) -> Self {
        WrappedProposal {
            account_id: proposal.account_id,
            proof_of_work: proposal.proof_of_work,
            is_approved: proposal.is_approved,
            is_reject: proposal.is_rejected,
        }
    }
}
