use std::env;

use axum::{
    extract::{State, Multipart},
    http::header::AUTHORIZATION,
    response::IntoResponse, Json, http::HeaderMap,
};
use reqwest::StatusCode;
use serde_json::json;
use tokio::fs::create_dir_all;

use crate::store::Store;
use crate::utils::{auth, media};


pub async fn upload_file(headers: HeaderMap, _store: State<Store>, mut multiplart: Multipart) -> impl IntoResponse {

    let authorization_header = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();

    let user_id = auth::get_user_id(authorization_header).await;


    while let Some(field) = multiplart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        create_dir_all(format!("{}/media/{}", env::current_dir().unwrap().to_string_lossy(), &user_id)).await;

        let path_file = format!("{}/media/{}/{}", env::current_dir().unwrap().to_string_lossy(), user_id, file_name);

        media::save_user_file(&path_file, data).await;
    }

    (StatusCode::CREATED, Json(json!({"status": true})))
}