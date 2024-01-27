use axum::{extract::Path, Json};
use uuid::Uuid;

pub(crate) async fn confirm(Path(registration_id): Path<Uuid>) -> Json<String> {
    Json(format!("Hello, {}!", registration_id))
}
