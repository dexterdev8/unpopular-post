use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Post {
    pub id: u64,
    pub owner_id: Addr,
    pub msg: String,
    pub likes: u64,
}

pub const POSTS: Item<Vec<Post>> = Item::new("unpopular_posts");
