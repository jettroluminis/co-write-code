mod web;

#[tokio::main]
async fn main() {
    web::main().await;
}
