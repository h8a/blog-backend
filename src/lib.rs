use std::{net::SocketAddr, str::FromStr};

use axum::{
    middleware,
    routing::{get, post, put, delete},
    Router, extract::DefaultBodyLimit,
};

use store::Store;

mod middleware_hooks;
mod resources;
mod types;
mod utils;
mod store;

async fn healthcheck() -> &'static str {
    "OK"
}

async fn db() -> Store {
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

    store
}

pub async fn router_app() -> Router {
    let app : Router = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/auth/login", post(resources::auth::login_user))
        .route("/auth/register", post(resources::auth::register_user))
        .route("/media/file/upload", post(resources::media::upload_file))
        .route("/media/file/:name_generated", get(resources::media::get_media))
        .route("/posts", post(resources::posts::create_posts))
        .route("/posts/:id", put(resources::posts::update_posts))
        .route("/posts/:id", delete(resources::posts::delete_posts))
        .with_state(db().await)
        .layer(DefaultBodyLimit::disable())
        .layer(middleware::from_fn(middleware_hooks::auth::authorization));

    app
}

pub async fn app(listener: &str) {
    let addr = SocketAddr::from_str(listener).unwrap();

    axum::Server::bind(&addr)
        .serve(router_app().await.into_make_service())
        .await
        .unwrap();
}