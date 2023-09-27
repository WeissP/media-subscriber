use crate::{
    docs::docs_routes,
    middlewares,
    routes::{self, youtube},
    store::{self, AppState, Store},
    FRONT_PUBLIC,
};
use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use axum_sessions::{async_session::SessionStore, SessionLayer};
use std::{env, sync::Arc};
use tower_http::{services::ServeDir, trace::TraceLayer};

// *********
// FRONT END
// *********
// Front end to server svelte build bundle, css and index.html from public folder
pub fn front_public_route() -> Router {
    let dir = env::var("FRONT_PUBLIC")
        .ok()
        .unwrap_or_else(|| FRONT_PUBLIC.to_string());
    Router::new()
        .fallback_service(
            ServeDir::new(dir).not_found_service(handle_error.into_service()),
        )
        .layer(TraceLayer::new_for_http())
}

#[allow(clippy::unused_async)]
async fn handle_error() -> (StatusCode, &'static str) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong accessing static files...",
    )
}

// ********
// BACK END
// ********
// Back end server built form various routes that are either public, require auth, or secure login
pub fn backend<Store: SessionStore>(
    session_layer: SessionLayer<Store>,
    shared_state: AppState,
) -> Router {
    let mut api = OpenApi::default();
    // could add tower::ServiceBuilder here to group layers, especially if you add more layers.
    // see https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    ApiRouter::new()
        .merge(back_public_route())
        .merge(back_auth_route())
        .merge(back_token_route(shared_state.clone()))
        .nest_api_service("/docs", docs_routes(shared_state.clone()))
        .finish_api(&mut api)
        .layer(session_layer)
        .layer(Extension(Arc::new(api)))
}

// *********
// BACKEND NON-AUTH
// *********
//
pub fn back_public_route() -> ApiRouter {
    ApiRouter::new()
        .route("/auth/session", get(routes::session::data_handler)) // gets session data
        .route("/auth/login", post(routes::login)) // sets username in session
        .route("/auth/logout", get(routes::logout)) // deletes username in session
        .route("/test", get(routes::not_implemented_route))
        .nest("/youtube", youtube::route())
}

// *********
// BACKEND SESSION
// *********
//
pub fn back_auth_route() -> Router {
    Router::new()
        .route("/secure", get(routes::session::handler))
        .route_layer(middleware::from_fn(middlewares::user_secure))
}

// *********
// BACKEND API
// *********
//
// invoked with State that stores API that is checked by the `middleware::auth`
pub fn back_token_route<S>(state: Arc<Store>) -> Router<S> {
    Router::new()
        .route("/api", get(routes::api::handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::auth,
        ))
        .with_state(state)
}
