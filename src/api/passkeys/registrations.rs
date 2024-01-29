use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{patch, post},
    Json, Router,
};
use uuid::Uuid;
use webauthn_rs::prelude::{RegisterPublicKeyCredential, Url};

use crate::{
    app,
    models::registration::{Model, RelyingParty, UserParams},
};

async fn create(
    State(context): State<app::Context>,
    Json(params): Json<UserParams>,
) -> impl IntoResponse {
    let relying_party = RelyingParty {
        name: context.config.relying_party_name.clone(),
        origin: Url::parse(&context.config.relying_party_origin).unwrap(),
    };
    let registration = Model::new(&context.database_connection, relying_party, params)
        .await
        .unwrap();
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from(serde_json::to_string(&registration).unwrap()))
        .unwrap()
}

async fn confirm(
    State(context): State<app::Context>,
    Path(registration_id): Path<Uuid>,
    Json(reg): Json<RegisterPublicKeyCredential>,
) -> Response<Body> {
    let relying_party = RelyingParty {
        name: context.config.relying_party_name.clone(),
        origin: Url::parse(&context.config.relying_party_origin).unwrap(),
    };
    let registration = Model::find_by_id(&context.database_connection, registration_id)
        .await
        .unwrap();
    let _passkey = registration
        .confirm(&context.database_connection, relying_party, &reg)
        .await
        .unwrap();
    StatusCode::NO_CONTENT.into_response()
}

pub(crate) fn router() -> Router<app::Context> {
    Router::new()
        .route("/passkeys/registrations", post(create))
        .route("/passkeys/registrations/:id", patch(confirm))
}
