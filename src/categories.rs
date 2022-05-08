use super::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Category {
    id: CategoryId,
    name: String,
    created: u64,
    pub num_posts: u64,
}

pub type CategoryId = String;

#[near_bindgen]
impl Dupwork {
    pub fn new_category(
        &mut self,
        topic_name: String,
    ) -> bool {
        let topic_id = topic_name.to_lowercase().replace(" ", "_");

        assert!(
            topic_name.len() <= MAX_TITLE_LENGTH,
            "Can not make a post title more than {} characters",
            MAX_TITLE_LENGTH
        );

        assert!(
            !self.categories.get(&topic_id.clone()).is_some(),
            "Topic already exists"
        );

        // let account_id = env::predecessor_account_id();
        // let storage_update = self.new_storage_update(account_id.clone());

        let topic = Category {
            id: topic_id.clone(),
            name: topic_name,
            created: env::block_timestamp().into(),
            num_posts: 0,
        };

        self.categories.insert(&topic_id.clone(), &topic);
        // self.finalize_storage_update(storage_update);
        return true;
    }
}
