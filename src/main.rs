use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};

mod resources;
mod store;
mod types;

async fn healthcheck() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let store = store::Store::new("postgres://test_user:test_password@localhost:5411/test1").await;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");

    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/auth/login", post(resources::auth::login_user))
        .route("/auth/register", post(resources::auth::register_user))
        .with_state(store);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_checkhealth() {
        let health = super::healthcheck().await;
        assert_eq!(health, "OK");
    }
}
