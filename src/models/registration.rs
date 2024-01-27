use chrono::prelude::*;
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webauthn_rs::{
    prelude::{
        CreationChallengeResponse, Passkey, PasskeyRegistration, RegisterPublicKeyCredential, Url,
    },
    WebauthnBuilder,
};

pub(crate) use super::entities::registrations::Model;
use super::entities::{prelude::Registrations, registrations};

#[derive(Debug)]
pub(crate) struct RelyingParty {
    pub name: String,
    pub origin: Url,
}

#[derive(Debug, Deserialize)]
pub(crate) struct UserParams {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct Registration {
    pub id: Uuid,
    pub creation_challenge: CreationChallengeResponse,
}

impl Model {
    pub(crate) async fn new(
        db: &DatabaseConnection,
        relying_party: RelyingParty,
        params: UserParams,
    ) -> Result<Registration, DbErr> {
        let rp_id = relying_party.origin.domain().unwrap();
        let webauthn = WebauthnBuilder::new(rp_id, &relying_party.origin)
            .map(|builder| builder.rp_name(&relying_party.name))
            .unwrap()
            .build()
            .unwrap();
        match webauthn.start_passkey_registration(
            params.id,
            &params.name,
            &params.display_name,
            None,
        ) {
            Ok((ccr, skr)) => {
                let skr_json = serde_json::to_value(skr).unwrap();
                let txn = db.begin().await?;
                let registration = registrations::ActiveModel {
                    user_id: ActiveValue::set(params.id),
                    state: ActiveValue::set(skr_json),
                    ..Default::default()
                }
                .insert(&txn)
                .await?;
                txn.commit().await?;
                Ok(Registration {
                    id: registration.id,
                    creation_challenge: ccr,
                })
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    pub(crate) async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<registrations::Model, DbErr> {
        Registrations::find_by_id(id)
            .one(db)
            .await
            .map(|opt| opt.unwrap())
    }

    pub(crate) async fn confirm(
        self,
        db: &DatabaseConnection,
        relying_party: RelyingParty,
        reg: &RegisterPublicKeyCredential,
    ) -> Result<Passkey, DbErr> {
        let state = serde_json::from_value::<PasskeyRegistration>(self.state.clone()).unwrap();
        let rp_id = relying_party.origin.domain().unwrap();
        let webauthn = WebauthnBuilder::new(rp_id, &relying_party.origin)
            .map(|builder| builder.rp_name(&relying_party.name))
            .unwrap()
            .build()
            .unwrap();
        let passkey = webauthn.finish_passkey_registration(reg, &state).unwrap();
        let txn = db.begin().await?;
        let mut registration = self.into_active_model();
        registration.confirmed_at = ActiveValue::set(Some(Utc::now().fixed_offset()));
        registration.update(&txn).await?;
        txn.commit().await?;
        Ok(passkey)
    }
}
