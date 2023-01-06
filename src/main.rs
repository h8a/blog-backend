use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router,
};
use store::Store;

mod resources;
mod store;
mod types;

async fn healthcheck() -> &'static str {
    "OK"
}

fn get_router(store: Store) -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/auth/login", post(resources::auth::login_user))
        .route("/auth/register", post(resources::auth::register_user))
        .with_state(store)
}

#[allow(dead_code)]
fn test_router() -> Router {
  Router::new()
      .route("/healthcheck", get(healthcheck))
      .route("/auth/login", post(resources::auth::login_user))
}


#[tokio::main]
async fn main() {
    let store = store::Store::new("postgres://test_user:test_password@localhost:5411/test1").await;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");

    let app = get_router(store);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test_helper::TestClient;

    #[tokio::test]
    async fn test_healthcheck_route() {
        let app = super::test_router();
        let client = TestClient::new(app);
        let res = client.get("/healthcheck").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "OK");

    }

    #[tokio::test]
    async fn test_healthcheck() {
        let health = super::healthcheck().await;
        assert_eq!(health, "OK");
    }
}
