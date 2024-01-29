#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    supapasskeys::app::start_server().await;
}
