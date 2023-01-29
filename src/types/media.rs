use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MediaId {
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Media {
    pub id: Option<MediaId>,
    pub name: String,
    pub name_generated: Uuid,
    pub content_type: String,
    pub path: String,
}