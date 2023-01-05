use axum::extract::State;
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::json;

use crate::store::Store;
use crate::types::auth::{UserAuth, RegisterUserAuth};

pub async fn login_user(Json(payload): Json<UserAuth>) -> impl IntoResponse {
    println!("{:?}", payload);
    (StatusCode::CREATED, Json(json!({"status": true})))
}

pub async fn register_user(store: State<Store>, Json(payload): Json<RegisterUserAuth>) -> impl IntoResponse {
    println!("{:?}", payload);

    match store.add_user(payload).await {
        Ok(data) => {
            println!("data: {:?}", data);
            (StatusCode::CREATED, Json(json!({"status": true})))
        },
        Err(_) => (StatusCode::BAD_REQUEST, Json(json!({"status": false})))
    }
}