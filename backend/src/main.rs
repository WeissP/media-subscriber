#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(missing_docs)]

use aide::{axum::ApiRouter, openapi::OpenApi};
use anyhow::Context;
use axum::Router;
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use clap::Parser;
use errors::RespError;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr, sync::Arc};
use tracing::log::warn;
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

use crate::{config::Config, store::AppState};

mod config;
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

    let config = Config::parse();

    let addr = config.socket_addr();
    let state = AppState::new(&config).await?;

    // combine the front and backend into server
    let app = ApiRouter::new()
        .merge(services::front_public_route(config.front_public()))
        .merge(services::backend(state));

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
