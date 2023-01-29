use std::{env, path::Path};

use axum::{
    extract::{State, Multipart},
    http::header::AUTHORIZATION,
    response::IntoResponse, Json, http::HeaderMap,
};
use reqwest::StatusCode;
use serde_json::json;
use tokio::fs::create_dir_all;
use uuid::Uuid;

use crate::store::Store;
use crate::utils::{auth, media};


pub async fn upload_file(headers: HeaderMap, store: State<Store>, mut multiplart: Multipart) -> impl IntoResponse {

    let authorization_header = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();

    let user_id = auth::get_user_id(authorization_header).await;

    let mut status_media = Some(json!({}));

    while let Some(field) = multiplart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        create_dir_all(format!("{}/media/{}", env::current_dir().unwrap().to_string_lossy(), &user_id)).await;

        let extension_file = file_name.split(".").collect::<Vec<&str>>()[1];
        let unique_name = Uuid::new_v4();

        let path_file = format!("{}/media/{}/{}.{}",
            env::current_dir().unwrap().to_string_lossy(),
            user_id,
            unique_name.to_string(),
            extension_file
        );

        media::save_user_file(&path_file, data).await;

        let file_db = match store.add_file(
            &file_name,
            unique_name,
            &content_type,
            &""
        ).await {
            Ok(file) => Ok(file),
            Err(_) => Err(false)
        };

        match file_db {
            Ok(file) => {
                status_media = Some(json!({
                    "status_media": true,
                    "file_name": file_name,
                    "content_type": content_type,
                    "name_generated": file.name_generated
                }))
            }
            Err(_) => {
                status_media = Some(json!({
                    "status_media": false,
                    "file_name": file_name,
                    "content_type": content_type,
                    "name_generated": "",
                }))
            }
        };
    }

    match status_media {
        Some(status) => {
            if status.get("status_media").unwrap() == true {
                let data_response = json!({
                    "status": true,
                    "data": {
                        "file_name": status.get("file_name"),
                        "name_generated": status.get("name_generated"),
                        "content_type": status.get("content_type"),
                    }
                });
                (StatusCode::CREATED, Json(data_response))
            } else {
                (StatusCode::CREATED, Json(json!({"status": false})))
            }
        },
        None => (StatusCode::CREATED, Json(json!({"status": false}))),
        _ => (StatusCode::CREATED, Json(json!({"status": false}))),
    }

}