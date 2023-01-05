use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router
};

mod resources;
mod types;

async fn healthcheck() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/auth/login", post(resources::auth::login_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
