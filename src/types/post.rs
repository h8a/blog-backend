// use chrono::serde::ts_seconds_option::deserialize as from_tsopt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct PostId {
    pub id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Post {
    pub id: Option<PostId>,
    pub title: String,
    pub body: String,
    pub slug: Option<String>,
    // #[serde(deserialize_with = "from_tsopt")]
    // pub created_on: Option<chrono::DateTime<chrono::Utc>>,
    pub created_on: Option<String>,
    pub user_id: Option<i32>
}

