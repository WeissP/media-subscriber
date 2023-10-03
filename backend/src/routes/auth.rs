use aide::axum::ApiRouter;
use axum::{extract::State, response::IntoResponse, Json};
use axum_login::{secrecy::SecretVec, AuthUser, PostgresStore};
use axum_sessions::{
    async_session::serde_json::json, extractors::WritableSession,
};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    db::{self, User},
    store::AppState,
    utils::fixed_str,
};

pub fn route(state: AppState) -> ApiRouter {
    ApiRouter::new().with_state(state)
    // .api_route("/login")
}

impl AuthUser<i32> for User {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.hashed_password.clone().into())
    }
}

type AuthContext =
    axum_login::extractors::AuthContext<i64, User, PostgresStore<User>>;

async fn login_handler(
    mut auth: AuthContext,
    State(state): State<AppState>,
    Json(login): Json<Login>,
) {
    // let user = db::get_user(state.db());
    // auth.login(&user).await.unwrap();
    todo!()
}

/// route to handle log in
#[allow(clippy::unused_async)]
#[allow(clippy::missing_panics_doc)]
pub async fn login(
    mut session: WritableSession,
    Json(login): Json<Login>,
) -> impl IntoResponse {
    tracing::info!("Logging in user: {}", login.username);

    if check_password(&login.username, &login.password) {
        session.insert("user_id", login.username).unwrap();
        Json(json!({"result": "ok"}))
    } else {
        Json(json!({"result": "error"}))
    }
}

/// route to handle log out
#[allow(clippy::unused_async)]
pub async fn logout(mut session: WritableSession) -> impl IntoResponse {
    let user = session.get_raw("user_id").unwrap_or_default();
    tracing::info!("Logging out user: {}", user);
    // drop session
    session.destroy();
    Json(json!({"result": "ok"}))
}

// assume all passwords work
const fn check_password(_username: &str, _password: &str) -> bool {
    true
}

#[derive(Deserialize, JsonSchema)]
pub struct Login {
    username: String,
    password: String,
}
