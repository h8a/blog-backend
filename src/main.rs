use std::net::SocketAddr;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use dotenv;

mod middleware_hooks;
mod resources;
mod types;
mod store;

async fn healthcheck() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {

    let store = store::Store::new(&format!("postgres://{}:{}@{}:{}/{}",
        dotenv::var("DB_USER").unwrap(),
        dotenv::var("DB_PASSWORD").unwrap(),
        dotenv::var("DB_HOST").unwrap(),
        dotenv::var("DB_PORT").unwrap(),
        dotenv::var("DB_NAME").unwrap())).await;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");

    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/auth/login", post(resources::auth::login_user))
        .route("/auth/register", post(resources::auth::register_user))
        .with_state(store)
        .layer(middleware::from_fn(middleware_hooks::auth::authorization));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
