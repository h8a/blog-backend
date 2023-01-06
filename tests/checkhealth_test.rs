
#[cfg(test)]
mod tests {
    use blog_backend; 
    use axum::http::StatusCode;
    use axum_test_helper::TestClient;
    

    #[tokio::test]
    async fn test_healthcheck_route() {
        let app = blog_backend::test_router();
        let client = TestClient::new(app);
        let res = client.get("/healthcheck").send().await;
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "OK");

    }

    #[tokio::test]
    async fn test_healthcheck() {
        let health = blog_backend::healthcheck().await;
        assert_eq!(health, "OK");
    }
}
