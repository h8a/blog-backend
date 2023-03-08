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
    pub created_on: Option<chrono::DateTime<chrono::Utc>>,
    pub user_id: Option<i32>
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReferencesPostsId {
    pub id: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReferencesPosts {
    pub id: Option<ReferencesPostsId>,
    pub name: String,
    pub url: String,
    pub created_on: Option<chrono::DateTime<chrono::Utc>>,
    pub post_id: Option<i32>,
    pub user_id: Option<i32>
}