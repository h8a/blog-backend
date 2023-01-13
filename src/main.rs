use blog_backend::app;

#[tokio::main]
async fn main() {
    app("127.0.0.1:3000").await;
}
