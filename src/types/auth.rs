use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserAuth {
    pub username: String,
    pub password: String,
}