use axum::{
    extract::{State, Path, Query},
    http::header::{AUTHORIZATION},
    Json,
    response::IntoResponse, 
    http::HeaderMap
};

use regex::Regex;
use reqwest::StatusCode;
use serde_json::json;

use crate::{store::Store, types::post::{Post, Pagination, ReferencesPosts}, utils::auth};


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


pub async fn delete_posts(Path(id): Path<i32>, store: State<Store>) -> impl IntoResponse {
    return match store.delete_posts(id).await {
        Ok(is_deleted) => {
            if is_deleted {
                (StatusCode::ACCEPTED, Json(json!({
                    "status": true,
                })))
            } else {
                (StatusCode::BAD_REQUEST, Json(json!({
                    "status": false,
                    "message": "Error to the try delete post"
                })))
            }
        },
        Err(e) => {
            println!("Error delete_post: {:?}", e);

            (StatusCode::BAD_REQUEST, Json(json!({
                "status": false,
                "message": e.1
            })))
        }
    };
}

pub async fn list_posts(pagination: Query<Pagination>, store: State<Store>) -> impl IntoResponse {
    let pagination: Pagination = pagination.0;

    let start = pagination.page as i32 - 1;
    let limit = pagination.per_page as i32;

    return match store.list_posts(start, limit).await {
        Ok(posts) => {

            (StatusCode::ACCEPTED, Json(json!({
                "status": true,
                "data": *axum::Json(posts)
            })))
        },
        Err(e) => {
            println!("Error list_post: {:?}", e);

            (StatusCode::BAD_REQUEST, Json(json!({
                "status": false,
                "message": e.1
            })))
        }
    };
}

pub async fn create_references_posts(headers: HeaderMap, store: State<Store>, Json(payload): Json<ReferencesPosts>) -> impl IntoResponse {

    let authorization_header = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
    let user_id = auth::get_user_id(authorization_header).await;

    return match store.create_references_posts(&payload.name, &payload.url, payload.post_id.unwrap(), user_id).await {
        Ok(reference_post) => {
            (StatusCode::ACCEPTED, Json(json!({
                "status": true,
                "data": {
                    "id": reference_post.id.unwrap().id,
                    "name": reference_post.name,
                    "url": reference_post.url,
                    "created_on": reference_post.created_on.unwrap().timestamp_millis(),
                    "post_id": reference_post.post_id.unwrap(),
                    "user_id": reference_post.user_id.unwrap()
                }
            })))
        },
        Err(e) => {
            println!("ERROR REFERENCES POSTS: {:?}", e);

            (StatusCode::BAD_REQUEST, Json(json!({
                "status": false,
                "message": e.1
            })))
        }
    };
}

pub async fn update_references_posts(Path(id): Path<i32>, store: State<Store>, Json(payload): Json<ReferencesPosts>) -> impl IntoResponse {
    return match store.update_references_posts(id, &payload.name, &payload.url).await {
        Ok(reference_post) => {
            (StatusCode::ACCEPTED, Json(json!({
                "status": true,
                "data": {
                    "id": reference_post.id.unwrap().id,
                    "name": reference_post.name,
                    "url": reference_post.url,
                    "created_on": reference_post.created_on.unwrap().timestamp_millis(),
                    "post_id": reference_post.post_id.unwrap(),
                    "user_id": reference_post.user_id.unwrap()
                }
            })))
        },
        Err(e) => {
            println!("ERROR REFERENCES POSTS: {:?}", e);

            (StatusCode::BAD_REQUEST, Json(json!({
                "status": false,
                "message": e.1
            })))
        }
    }
}

pub async fn delete_references_posts(Path(id): Path<i32>, store: State<Store>) -> impl IntoResponse {
    return match store.delete_references_posts(id).await {
        Ok(is_deleted) => {
            if is_deleted {
                (StatusCode::ACCEPTED, Json(json!({
                    "status": true,
                })))
            } else {
                (StatusCode::BAD_REQUEST, Json(json!({
                    "status": false,
                    "message": "Error to the try delete reference"
                })))
            }
        },
        Err(e) => {
            println!("Error delete_reference_post: {:?}", e);

            (StatusCode::BAD_REQUEST, Json(json!({
                "status": false,
                "message": e.1
            })))
        }
    }
}

pub async fn lists_references_posts(Path(id): Path<i32>, store: State<Store>) -> impl IntoResponse {
    return match store.lists_references_posts(id).await {
        Ok(references_posts) => {

            (StatusCode::ACCEPTED, Json(json!({
                "status": true,
                "data": *axum::Json(references_posts)
            })))
        },
        Err(e) => {
            println!("Error list_references_post: {:?}", e);

            (StatusCode::BAD_REQUEST, Json(json!({
                "status": false,
                "message": e.1
            })))
        }
    };
}