use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserAuth {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterUserAuth {
    pub username: String,
    pub password: String,
    pub name: String,
    pub last_name: String,
    pub surname: Option<String>,
    pub picture: Option<String>,
}