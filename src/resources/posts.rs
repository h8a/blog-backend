use axum::{
    extract::State,
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
    println!("{:?}", payload);
    let authorization_header = headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
    let user_id = auth::get_user_id(authorization_header).await;

    let re = Regex::new(r"\s+").unwrap();
    let slug = re.replace_all(&payload.title, "-").to_string();

    // let post = Post {
    //     title: payload.title,
    //     body: payload.body,
    //     slug: Some(slug),
    //     user_id: Some(user_id),
    //     id: None,
    //     create_on: None,
    // };

    // println!("{:?}", post);

    match store.create_posts(&payload.title, &payload.body, &slug, user_id).await {
        Ok(post) => {
            println!("POST: {:?}", post);
        },
        Err(e) => {
            println!("Error: {:?}", e)
        }
    }

    (StatusCode::CREATED, Json(json!({
        "status": true
    })))
}

