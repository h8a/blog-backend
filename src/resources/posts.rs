use axum::{
    extract::{State, Path},
    http::header::{AUTHORIZATION},
    Json,
    response::IntoResponse, 
    http::HeaderMap
};

use regex::Regex;
use reqwest::StatusCode;
use serde_json::json;

use crate::{store::Store, types::post::Post, utils::auth};


pub async fn create_posts(headers: HeaderMap, store: State<Store>, Json(payload): Json<Post>) -> impl IntoResponse {

    let authorization_header = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
    let user_id = auth::get_user_id(authorization_header).await;

    let re = Regex::new(r"\s+").unwrap();
    let slug = re.replace_all(&payload.title, "-").to_string();

    return match store.create_posts(&payload.title, &payload.body, &slug, user_id).await {
        Ok(post) => {
            (StatusCode::CREATED, Json(json!({
                "status": true,
                "data": {
                    "id": post.id.unwrap().id,
                    "title": post.title,
                    "body": post.body,
                    "slug": post.slug,
                    "created_on": post.created_on.unwrap().timestamp_millis(),
                    "user_id": post.user_id
                }
            })))
        },
        Err(e) => {
            (StatusCode::BAD_REQUEST, Json(json!({
                "status": false,
                "message": e.1
            })))
        }
    };
}

pub async fn update_posts(Path(id): Path<i32>, store: State<Store>, Json(payload): Json<Post>) -> impl IntoResponse {
    let post_db = match store.get_post(id).await {
        Ok(post) => Ok(post),
        Err(e) => Err(e)
    };

    return match post_db {
        Ok(post_db) => {

            let title=   if payload.title == post_db.title { post_db.title } else { payload.title };
            let body =   if payload.body == post_db.body { post_db.body } else { payload.body };

            let re = Regex::new(r"\s+").unwrap();
            let slug = re.replace_all(&title, "-").to_string();

            return match store.update_posts(id, &title, &body, &slug).await {
                Ok(post) => {
                    (StatusCode::CREATED, Json(json!({
                        "status": true,
                        "data": {
                            "id": post.id.unwrap().id,
                            "title": post.title,
                            "body": post.body,
                            "slug": post.slug,
                            "created_on": post.created_on.unwrap().timestamp_millis(),
                            "user_id": post.user_id
                        }
                    })))
                },
                Err(e) => {
                    (StatusCode::BAD_REQUEST, Json(json!({
                        "status": false,
                        "message": e.1
                    })))
                }
            };
        },
        Err(e) => {
            println!("Error update_post: {:?}", e);

            (StatusCode::BAD_REQUEST, Json(json!({
                "status": false,
                "message": e.1
            })))
        }
    };
}

