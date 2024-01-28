use axum::{
    extract::Path,
    routing::{patch, post},
    Json, Router,
};
use uuid::Uuid;

use crate::app;

async fn create() -> Json<String> {
    Json("Hello, Authentication!".to_string())
}

async fn confirm(Path(authentication_id): Path<Uuid>) -> Json<String> {
    Json(format!("Hello, {}!", authentication_id))
}

pub(crate) fn router() -> Router<app::Context> {
    Router::new()
        .route("/passkeys/authentications", post(create))
        .route("/passkeys/authentications/:id", patch(confirm))
}
