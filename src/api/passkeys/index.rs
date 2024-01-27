use axum::{extract::State, Json};
use webauthn_rs::prelude::Url;

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
