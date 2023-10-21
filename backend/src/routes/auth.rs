use aide::axum::ApiRouter;
use anyhow::{anyhow, bail, Context};
use argon2::{
    password_hash::{self, SaltString},
    Argon2, PasswordHash, PasswordHasher,
};
use axum::{extract::State, response::IntoResponse, Json};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    db::{self},
    errors::{RespError, Response},
    store::AppState,
    utils::fixed_str,
};

pub fn route(state: AppState, secret: &[u8]) -> ApiRouter {
    ApiRouter::new().with_state(state)
    // .layer(AuthLayer::new(user_store, secret))
    // .api_route("/login")
}

// async fn login_handler(
//     mut auth: AuthContext,
//     State(state): State<AppState>,
//     Json(login): Json<Login>,
// ) -> Response<()> {
//     match db::get_user(state.db(), &login.username).await? {
//         Some(user) => auth.login(&user).await.unwrap(),
//         None => todo!(),
//     };
//     Ok(())
// }

async fn hash_password(password: String) -> anyhow::Result<String> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    Ok(
        tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
            let salt = SaltString::generate(rand::thread_rng());
            let pw_hash = Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| anyhow!("failed to generate password hash: {}", e))?
                .to_string();
            Ok(pw_hash)
        })
        .await
        .context("panic in generating password hash")??,
    )
}

async fn verify_password(password: String, password_hash: String) -> Response<()> {
    let rs = tokio::task::spawn_blocking(move || -> password_hash::Result<()> {
        let hash = PasswordHash::new(&password_hash)?;
        hash.verify_password(&[&Argon2::default()], password)
    })
    .await
    .context("panic in generating password hash")?;
    rs.map_err(|e| match e {
        password_hash::Error::Password => RespError::Unauthorized,
        _ => anyhow!("failed to generate password hash: {}", e).into(),
    })
}

#[derive(Deserialize, JsonSchema)]
pub struct Login {
    username: String,
    password: String,
}
