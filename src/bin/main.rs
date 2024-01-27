#[tokio::main]
async fn main() {
    supapasskeys::app::start_server().await;
}
