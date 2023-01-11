use axum::extract::State;
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::json;

use crate::store::Store;
use crate::types::auth::{UserAuth, RegisterUserAuth};
use crate::utils::security::{jwt_encode};

pub async fn login_user(store: State<Store>,Json(payload): Json<UserAuth>) -> impl IntoResponse {
    let user = store.get_user_by_username(&payload.username).await.unwrap();
    println!("USER: {:?}", user);
    println!("PAYLOAD: {:?}", payload);
    (StatusCode::CREATED, Json(json!({"status": true})))
}

pub async fn register_user(store: State<Store>, Json(payload): Json<RegisterUserAuth>) -> impl IntoResponse {

    match store.add_user(payload).await {
        Ok(data) => {

            let token = jwt_encode(&data.id.unwrap().id);

            (StatusCode::CREATED, Json(json!({
                "status": true,
                "data": {
                    "token": token
                }
            })))
        },
        Err(_) => (StatusCode::BAD_REQUEST, Json(json!({"status": false})))
    }
}