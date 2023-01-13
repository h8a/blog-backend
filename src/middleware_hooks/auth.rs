use axum::{
    http::{
        Request,
        StatusCode,
        header::AUTHORIZATION
    }, 
    middleware::Next, 
    response::Response
};

use crate::utils::security::jwt_decode;

pub async fn authorization<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match req.uri().path() {
        "/auth/register" | "/auth/login" | "/healthcheck" => {
            Ok(next.run(req).await)
        },
        _ => {
            let header_is_valid =  header_is_valid(auth_header);
            if header_is_valid {
                Ok(next.run(req).await)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        },
    }
}

fn header_is_valid(header: Option<&str>) -> bool {
    match header {
        Some(token) => {
            let token: Vec<&str> = token.split(" ").collect();
            let suffix = token[0];
            if suffix == dotenv::var("JWT_PREFIX").unwrap() {
                token_is_valid(token[1])
            } else {
                false
            }
        },
        None => false
    }
}

fn token_is_valid(token: &str) -> bool {
    let is_valid =  jwt_decode(token);
    match is_valid {
        Ok(_) => true,
        Err(_) => false
    }
}