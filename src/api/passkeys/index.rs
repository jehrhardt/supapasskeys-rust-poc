use axum::Json;

pub(crate) async fn create() -> Json<String> {
    Json("Hello, World!".to_string())
}
