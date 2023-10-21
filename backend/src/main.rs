#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(missing_docs)]

use aide::{axum::ApiRouter, openapi::OpenApi};
use anyhow::Context;
use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, BoxError, Extension,
    Router,
};
use clap::Parser;
use errors::RespError;
use std::{env, net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::log::warn;
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

use crate::{config::Config, docs::docs_routes, store::AppState};

mod config;
mod cornucopia;
mod db;
mod docs;
mod errors;
mod extractors;
// pub mod middlewares;
pub mod routes;
mod services;
mod store;
mod utils;

/// Server that is split into a Frontend to serve static files (Svelte) and Backend
/// Backend is further split into a non authorized area and a secure area
/// The Back end is using 2 middleware: sessions (managing session data) and user_secure (checking for authorization)
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init log
    let sub = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_level(true);
    match env::var("MS_LOG_DIR").ok() {
        Some(dir) => {
            let file_appender =
                tracing_appender::rolling::hourly(dir, "media_subscriber.log");
            let (non_blocking, _guard) =
                tracing_appender::non_blocking(file_appender);
            sub.with_writer(non_blocking).init();
        }
        None => sub.init(),
    };

    let config = Config::parse();

    let addr = config.socket_addr();
    let state = AppState::new(&config).await?;
    let session_store = MemoryStore::default();
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(
            SessionManagerLayer::new(session_store)
                .with_secure(true)
                .with_max_age(time::Duration::DAY),
        );

    let mut api = OpenApi::default();

    // combine the front and backend into server
    let app = ApiRouter::new()
        .merge(services::front_public_route(config.front_public()))
        .merge(services::backend())
        .merge(docs_routes(state.clone()))
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)))
        .with_state(state)
        .layer(session_service);

    tracing::info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}
