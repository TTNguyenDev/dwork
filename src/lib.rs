use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{ValidAccountId, WrappedBalance, WrappedDuration, WrappedTimestamp};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, ext_contract, near_bindgen, setup_alloc, Balance, Duration, Gas, PanicOnDefault, Promise,
    PromiseResult, Timestamp,
};
use std::convert::TryFrom;

pub use crate::categories::*;
pub use crate::constants::*;
pub use crate::ext::*;
pub use crate::json_types::*;
pub use crate::types::*;
pub use crate::views::*;

mod categories;
mod constants;
mod ext;
mod json_types;
mod types;
mod views;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Dupwork {
    tasks_recores: UnorderedMap<TaskId, Task>,
    users: LookupMap<ValidAccountId, User>,
    categories: UnorderedMap<CategoryId, Category>,
}

#[near_bindgen]
impl Dupwork {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized",);

        Self {
            tasks_recores: UnorderedMap::new(b"tasks_recores".to_vec()),
            users: LookupMap::new(b"users".to_vec()),
            categories: UnorderedMap::new(b"categories".to_vec()),
        }
    }

    #[payable]
    pub fn register(&mut self, requester: bool) {
        assert!(
            env::attached_deposit() == REGISTER_BOND,
            "Send exactly {:?} Near to register",
            REGISTER_BOND
        );

        let account_id = ValidAccountId::try_from(env::predecessor_account_id()).unwrap();
        let mut current_jobs_prefix = Vec::with_capacity(33);
        current_jobs_prefix.push(b'c');
        current_jobs_prefix.extend(env::sha256(env::predecessor_account_id().as_bytes()));

        let mut completed_jobs_prefix = Vec::with_capacity(33);
        completed_jobs_prefix.push(b'd');
        completed_jobs_prefix.extend(env::sha256(env::predecessor_account_id().as_bytes()));

        if requester {
            let user = User {
                account_id: account_id.clone(),
                user_type: UserType::Requester {
                    total_transfered: 0,
                    current_requests: 0,
                },
                bio: "A member of dWork".to_string(),
                completed_jobs: UnorderedSet::new(completed_jobs_prefix),
                current_jobs: UnorderedSet::new(current_jobs_prefix),
            };

            self.users.insert(&account_id, &user);
        } else {
            let user = User {
                account_id: account_id.clone(),
                bio: "A member of dWork".to_string(),
                user_type: UserType::Worker {
                    total_received: 0,
                    current_applies: 0,
                },
                completed_jobs: UnorderedSet::new(completed_jobs_prefix),
                current_jobs: UnorderedSet::new(current_jobs_prefix),
            };

            self.users.insert(&account_id, &user);
        }
    }

    pub fn leave(&mut self) {
        let account_id = ValidAccountId::try_from(env::predecessor_account_id()).unwrap();
        self.users
            .get(&account_id)
            .expect("You are not a member of dupwork");

        Promise::new(env::predecessor_account_id()).transfer(REGISTER_BOND);

        self.users.remove(&account_id);
    }

    /// Requester sections:
    #[payable]
    pub fn new_task(
        &mut self,
        title: String,
        description: String,
        price: WrappedBalance,
        max_participants: u16,
        duration: WrappedDuration,
        category_id: CategoryId,
    ) {
        //TODO: Maximum deposit
        //
        let owner = ValidAccountId::try_from(env::predecessor_account_id()).unwrap();
        let mut user = self
            .users
            .get(&owner)
            .expect("You are not a member of dupwork");

        let unwrap_balance: Balance = price.into();
        let amount_need_to_pay: Balance = (max_participants as u128)
            .checked_mul(unwrap_balance)
            .expect("Cannot calculate total amount");

        assert!(
            env::attached_deposit() == amount_need_to_pay,
            "Attach exactly {} yoctoNear",
            amount_need_to_pay
        );

        assert!(
            env::attached_deposit() >= MINIMUM_PRICE_PER_TASK
                && env::attached_deposit() <= MAXIMUM_PRICE_PER_TASK,
            "Total amount for each task must be in a range from {} to {}",
            MINIMUM_PRICE_PER_TASK,
            MAXIMUM_PRICE_PER_TASK
        );

        match user.user_type {
            UserType::Worker { .. } => panic!("Only requester can create a task"),
            UserType::Requester {
                total_transfered,
                current_requests,
            } => {
                assert!(
                    description.len() <= MAXIMUM_DESCRIPTION_LENGTH,
                    "Description too long"
                );

                assert!(
                    max_participants <= MAXIMUM_PROPOSAL_AT_ONE_TIME,
                    "Only accept {} at one time",
                    MAXIMUM_PROPOSAL_AT_ONE_TIME
                );

                let task_id = env::predecessor_account_id() + "_" + &env::block_index().to_string();

                assert!(
                    !self.tasks_recores.get(&task_id).is_some(),
                    "Can't post twice per block"
                );

                let mut proposal_prefix = Vec::with_capacity(33);
                // Adding unique prefix.
                proposal_prefix.push(b'p');
                proposal_prefix.extend(env::sha256(task_id.as_bytes()));

                let unwrap_duration: Duration = duration.into();

                let task = Task {
                    owner: owner.clone(),
                    title,
                    description,
                    price: price.into(),
                    max_participants,
                    proposals: UnorderedMap::new(proposal_prefix),
                    created_at: env::block_timestamp(),
                    available_until: env::block_timestamp() + unwrap_duration,
                    category_id: category_id.clone(),
                };

                //Update num_posts in category
                if let Some(mut category) = self.categories.get(&category_id) {
                    category.num_posts += 1;
                    self.categories.insert(&category_id, &category);
                } else {
                    panic!("Not found your category");
                }

                self.tasks_recores.insert(&task_id, &task);
                //Update user current requests
                user.user_type = UserType::Requester {
                    total_transfered,
                    current_requests: current_requests + 1,
                };
                user.current_jobs.insert(&task_id);
                self.users.insert(&owner, &user);
            }
        }
    }

    pub fn approve_work(&mut self, task_id: TaskId, worker_id: ValidAccountId) {
        let task = self.tasks_recores.get(&task_id).expect("Job not exist");

        let owner = ValidAccountId::try_from(env::predecessor_account_id()).unwrap();
        assert!(task.owner == owner, "Only owner can approve proposal");

        let proposal = task.proposals.get(&worker_id).expect("Not found proposal");
        let beneficiary_id = proposal.account_id;
        let amount_to_transfer = task.price.into();

        assert!(!proposal.is_rejected, "You already rejected this worker!!");
        // Make a transfer to the worker
        Promise::new(beneficiary_id.to_string())
            .transfer(amount_to_transfer + SUBMIT_BOND)
            .then(ext_self::on_transferd(
                task_id,
                beneficiary_id,
                amount_to_transfer,
                &env::current_account_id(),
                0,
                DEFAULT_GAS_TO_PAY,
            ));
    }

    pub fn reject_work(&mut self, task_id: TaskId, worker_id: ValidAccountId) {
        let mut task = self.tasks_recores.get(&task_id).expect("Job not exist");

        let beneficiary_id = ValidAccountId::try_from(env::predecessor_account_id()).unwrap();
        assert!(
            task.owner == beneficiary_id,
            "Only owner can reject proposal"
        );

        let mut proposal = task.proposals.get(&worker_id).expect("Not found proposal");
        assert!(!proposal.is_approved, "You already approved this worker!!");
        proposal.is_rejected = true;

        task.proposals.insert(&worker_id, &proposal);
        self.tasks_recores.insert(&task_id, &task);

        // let amount_to_transfer: Balance = task.price.into();
        // Promise::new(beneficiary_id.to_string()).transfer(amount_to_transfer + SUBMIT_BOND);
    }

    pub fn mark_task_as_completed(&mut self, task_id: TaskId) {
        let task = self.tasks_recores.get(&task_id).expect("Job not exist");

        let beneficiary_id = ValidAccountId::try_from(env::predecessor_account_id()).unwrap();
        assert!(
            task.owner == beneficiary_id,
            "Only owner can reject proposal"
        );

        assert!(
            task.proposals
                .iter()
                .filter(|(_k, v)| v.is_approved == false && v.is_rejected == false)
                .count()
                == 0,
            "Some work remains unchecked"
        );

        let completed_proposals_count = task
            .proposals
            .iter()
            .filter(|(_k, v)| v.is_approved == true)
            .count();

        let refund: u64 = (task.max_participants as u64) - task.proposals.len();

        let amount_to_transfer = (task.price as u128)
            .checked_mul(refund.into())
            .expect("Can not calculate amount to refund");
        if completed_proposals_count < task.max_participants as usize {
            assert!(
                task.available_until < env::block_timestamp(),
                "This request is not expire, you can not mark it completed!"
            );

            Promise::new(beneficiary_id.to_string()).transfer(amount_to_transfer);
        }

        let mut owner = self.users.get(&beneficiary_id).expect("Not found owner");
        owner.completed_jobs.insert(&task_id);
        owner.current_jobs.remove(&task_id);

        if let UserType::Requester {
            total_transfered,
            current_requests,
        } = owner.user_type
        {
            assert!(current_requests > 0, "Current requests is zero!");
            owner.user_type = UserType::Requester {
                total_transfered: total_transfered + amount_to_transfer,
                current_requests: current_requests - 1,
            };

            self.users.insert(&beneficiary_id, &owner);
        }
        // panic!("Some err need to check!");
    }

    #[payable]
    pub fn submit_work(&mut self, task_id: String, proof: String) {
        assert!(
            env::attached_deposit() == SUBMIT_BOND,
            "Send exactly {:?} Near to register",
            SUBMIT_BOND
        );

        let mut task = self.tasks_recores.get(&task_id).expect("Job not exist");

        assert!(
            task.available_until > env::block_timestamp(),
            "This request is expire!"
        );

        assert!(
            task.proposals
                .iter()
                .filter(|(_k, v)| v.is_approved)
                .count()
                < task.max_participants as usize,
            "Full approved participants"
        );

        //TODO increase worker current task

        let worker_id = ValidAccountId::try_from(env::predecessor_account_id()).unwrap();

        let mut worker = self.users.get(&worker_id).expect("User not found");

        if let UserType::Worker {
            total_received,
            current_applies,
        } = worker.user_type
        {
            worker.user_type = UserType::Worker {
                total_received,
                current_applies: current_applies + 1,
            };

            worker.current_jobs.insert(&task_id);
            self.users.insert(&worker_id, &worker);
        }

        let proposal = Proposal {
            account_id: worker_id.clone(),
            proof_of_work: proof,
            is_approved: false,
            is_rejected: false,
        };

        task.proposals.insert(&worker_id, &proposal);
        self.tasks_recores.insert(&task_id, &task);
    }

    //Account logic
    pub fn update_bio(&mut self, bio: String) {
        let account_id = ValidAccountId::try_from(env::predecessor_account_id()).unwrap();
        let mut user = self.users.get(&account_id).expect("User not found");

        user.bio = bio;
        self.users.insert(&account_id, &user);
    }
}
