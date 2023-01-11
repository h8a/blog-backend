use axum::{http::{Request, StatusCode, header::AUTHORIZATION}, middleware::Next, response::Response};

pub async fn authorization<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    println!("AUTH_HEADER: {:?}", auth_header);

    Ok(next.run(req).await)

    // match auth_header {
    //     Some(auth_header) if token_is_valid(auth_header) => {
    //         Ok(next.run(req).await)
    //     }
    //     _ => Err(StatusCode::UNAUTHORIZED),
    // }
}

fn token_is_valid(token: &str) -> bool {
    println!("{}", token);
    true
}