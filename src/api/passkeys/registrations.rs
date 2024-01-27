use axum::{
    body::Body,
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use webauthn_rs::prelude::{RegisterPublicKeyCredential, Url};

use crate::{
    app,
    models::registration::{Model, Registration, RelyingParty, UserParams},
};

pub(crate) async fn create(
    State(context): State<app::Context>,
    Json(params): Json<UserParams>,
) -> Json<Registration> {
    let relying_party = RelyingParty {
        name: context.config.relying_party_name.clone(),
        origin: Url::parse(&context.config.relying_party_origin).unwrap(),
    };
    let registration = Model::new(&context.database_connection, relying_party, params)
        .await
        .unwrap();
    Json(registration)
}

pub(crate) async fn confirm(
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
