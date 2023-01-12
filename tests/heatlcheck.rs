// use std::net::TcpListener;
// use std::{net::SocketAddr, str::FromStr};

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client.get("http://127.0.0.1:3000/healthcheck")
        .send()
        .await
        .expect("Failed to execute requests.");
    
    // println!("{:?}", response.text_with_charset("utf-8").await);

    assert!(response.status().is_success());
    // assert!(true);
    // assert_eq!("OK", response.text().await.unwrap());
}

fn spawn_app() {
    // let listener = SocketAddr::from_str("127.0.0.1:0").unwrap();

    // let port = listener.port();

    let server = blog_backend::app("127.0.0.1:3000");

    let _ = tokio::spawn(server);
}