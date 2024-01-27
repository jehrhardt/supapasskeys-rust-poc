use axum::{extract::Path, Json};
use uuid::Uuid;

pub(crate) async fn create() -> Json<String> {
    Json("Hello, Authentication!".to_string())
}

pub(crate) async fn confirm(Path(authentication_id): Path<Uuid>) -> Json<String> {
    Json(format!("Hello, {}!", authentication_id))
}
