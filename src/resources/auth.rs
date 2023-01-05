use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::json;

use crate::types::auth::{UserAuth, RegisterUserAuth};

pub async fn login_user(Json(payload): Json<UserAuth>) -> impl IntoResponse {
    println!("{:?}", payload);
    (StatusCode::CREATED, Json(json!({"status": true})))
}

pub async fn register_user(Json(payload): Json<RegisterUserAuth>) -> impl IntoResponse {
    println!("{:?}", payload);
    (StatusCode::CREATED, Json(json!({"status": true})))
}