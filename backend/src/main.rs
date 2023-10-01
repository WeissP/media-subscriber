#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(missing_docs)]

use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::Router;
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use errors::AppError;
use std::{env, net::SocketAddr, sync::Arc};
use tracing::log::warn;
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

mod docs;
mod errors;
mod extractors;
pub mod middlewares;
pub mod routes;
mod services;
mod store;
mod utils;

pub type Result<T> = std::result::Result<T, AppError>;

// SETUP Constants
const SESSION_COOKIE_NAME: &str = "media_subscriber_session";
const FRONT_PUBLIC: &str = "./frontend/dist";
const SERVER_PORT: &str = "8080";
const SERVER_HOST: &str = "127.0.0.1";

/// Server that is split into a Frontend to serve static files (Svelte) and Backend
/// Backend is further split into a non authorized area and a secure area
/// The Back end is using 2 middleware: sessions (managing session data) and user_secure (checking for authorization)
#[tokio::main]
async fn main() {
    // init log
    let sub = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_level(true);
    match env::var("MA_LOG_DIR").ok() {
        Some(dir) => {
            let file_appender =
                tracing_appender::rolling::hourly(dir, "media_subscriber.log");
            let (non_blocking, _guard) =
                tracing_appender::non_blocking(file_appender);
            sub.with_writer(non_blocking).init();
        }
        None => sub.init(),
    };

    // configure server from environmental variables
    let (port, host, secret) = from_env();

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Can not parse address and port");

    // create store for backend.  Stores an api_token.
    let shared_state = Arc::new(store::Store::new("123456789"));

    // setup up sessions and store to keep track of session information
    let session_layer = SessionLayer::new(MemoryStore::new(), secret.as_bytes())
        .with_cookie_name(SESSION_COOKIE_NAME);

    // combine the front and backend into server
    let app = ApiRouter::new()
        .merge(services::front_public_route())
        .merge(services::backend(session_layer, shared_state));

    tracing::info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}

// Variables from Environment or default to configure server
// port, host, secret
fn from_env() -> (String, String, String) {
    if env::var("MA_SERVER_SECRET").is_err() {
        warn!("env var SERVER_SECRET should be set and unique (64 bytes long)");
    }
    (
        env::var("MA_SERVER_PORT")
            .ok()
            .unwrap_or_else(|| SERVER_PORT.to_string()),
        env::var("MA_SERVER_HOST")
            .ok()
            .unwrap_or_else(|| SERVER_HOST.to_string()),
        env::var("MA_SERVER_SECRET").ok().unwrap_or_else(|| {
            "this needs to be 64bytes. recommended that you set Secret instead of fixed value"
                .to_string()
        }),
    )
}
