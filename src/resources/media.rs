use axum::{
    extract::{State, Multipart}, 
    response::IntoResponse, Json,
};
use reqwest::StatusCode;
use serde_json::json;

use crate::store::Store;


pub async fn upload_file(_store: State<Store>, mut multiplart: Multipart) -> impl IntoResponse {
    while let Some(mut field) = multiplart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{}` (`{}`: `{}`) is {} bytes",
            name,
            file_name,
            content_type,
            data.len()
        );
    }

    (StatusCode::ACCEPTED, Json(json!({"status": true})))
}