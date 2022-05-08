use crate::*;
use near_sdk::serde_json::{json, Value};

#[near_bindgen]
impl Dupwork {
    pub fn available_tasks(&self, from_index: u64, limit: u64) -> Vec<(TaskId, WrappedTask)> {
        let tasks_id = self.tasks_recores.keys_as_vector();

        let from = if tasks_id.len() > (limit + from_index) {
            tasks_id.len() - limit - from_index
        } else {
            0
        };

        let to = if tasks_id.len() > from_index {
            tasks_id.len() - from_index
        } else {
            0
        };

        (from..to)
            .map(|index| {
                let task_id = tasks_id.get(index as u64).unwrap();
                let task = self.tasks_recores.get(&task_id.clone()).unwrap();
                (task_id.clone(), WrappedTask::from(task))
            })
            .rev()
            .collect()
    }

    pub fn current_tasks(
        &self,
        account_id: ValidAccountId,
        from_index: u64,
        limit: u64,
    ) -> Vec<(TaskId, WrappedTask)> {
        let tasks_id = self
            .users
            .get(&account_id)
            .expect("User not found")
            .current_jobs
            .to_vec();

        let from = if tasks_id.len() as u64 > (limit + from_index) {
            tasks_id.len() as u64 - limit - from_index
        } else {
            0
        };

        let to = if tasks_id.len() as u64 > from_index {
            tasks_id.len() as u64 - from_index
        } else {
            0
        };

        (from..to)
            .map(|index| {
                let key = tasks_id.get(index as usize).unwrap();
                (
                    key.clone(),
                    WrappedTask::from(self.tasks_recores.get(&key).unwrap()),
                )
            })
            .rev()
            .collect()
    }

    pub fn completed_tasks(
        &self,
        account_id: ValidAccountId,
        from_index: u64,
        limit: u64,
    ) -> Vec<(TaskId, WrappedTask)> {
        let tasks_id = self
            .users
            .get(&account_id)
            .expect("User not found")
            .completed_jobs
            .to_vec();

        let from = if tasks_id.len() as u64 > (limit + from_index) {
            tasks_id.len() as u64 - limit - from_index
        } else {
            0
        };

        let to = if tasks_id.len() as u64 > from_index {
            tasks_id.len() as u64 - from_index
        } else {
            0
        };

        (from..to)
            .map(|index| {
                let key = tasks_id.get(index as usize).unwrap();
                (
                    key.clone(),
                    WrappedTask::from(self.tasks_recores.get(&key).unwrap()),
                )
            })
            .rev()
            .collect()
    }

    pub fn user_info(&self, account_id: ValidAccountId) -> Value {
        self.users
            .get(&account_id)
            .map(|v| {
                json!({
                    "account_id": v.account_id,
                    "bio": v.bio,
                    "user_type": WrappedUserType::from(v.user_type),
                    "completed_jobs": v.completed_jobs.to_vec()
                })
            })
            .expect("Canot map user to json")
    }

    pub fn task_by_id(&self, task_id: TaskId) -> WrappedTask {
        self.tasks_recores
            .get(&task_id)
            .map(|v| WrappedTask::from(v))
            .expect("Task not found")
    }

    pub fn tasks_by_ids(&self, ids: Vec<String>) -> Vec<(String, WrappedTask)> {
        ids.iter()
            .map(|id| (id.clone(), self.task_by_id(id.to_string())))
            .collect()
    }

    //Get categories
    pub fn categories(&self, from_index: u64, limit: u64) -> Vec<Category> {
        let category_ids = self.categories.keys_as_vector();

        (from_index..std::cmp::min(from_index + limit, category_ids.len()))
            .map(|index| {
                let category_id = category_ids.get(index).unwrap();
                self.categories.get(&category_id).unwrap()
            })
            .collect()
    }

    pub fn maximum_participants_per_task(&self) -> u16 {
        MAXIMUM_PROPOSAL_AT_ONE_TIME
    }
}
