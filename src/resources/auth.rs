use axum::extract::State;
use axum::{Json, response::IntoResponse, http::StatusCode};
use serde_json::json;

use crate::store::Store;
use crate::types::auth::{UserAuth, RegisterUserAuth};
use crate::utils::security::{jwt_encode, encode_password, decode_password};

pub async fn login_user(store: State<Store>,Json(payload): Json<UserAuth>) -> impl IntoResponse {

    match store.get_user_by_username(&payload.username).await {
        Ok(data) => {

            let is_verify = decode_password(&data.password, &payload.password.as_bytes());

            match is_verify {
                true => {
                    let token = jwt_encode(&data.id.unwrap().id);

                    (StatusCode::ACCEPTED, Json(json!({
                        "status": true,
                        "data": {
                            "token": token
                        }
                    })))
                },
                false => {
                    (StatusCode::BAD_REQUEST, Json(json!({
                        "status": false,
                        "message": "the credentials was wrong",
                    })))
                }
            }
        },
        Err(_) => (StatusCode::BAD_REQUEST, Json(json!({"status": false})))
    }
}

pub async fn register_user(store: State<Store>, Json(payload): Json<RegisterUserAuth>) -> impl IntoResponse {

    let password_hash = encode_password(&payload.password.as_bytes());

    let user = RegisterUserAuth {
        password: password_hash,
        ..payload
    };

    match store.add_user(user.clone()).await {
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