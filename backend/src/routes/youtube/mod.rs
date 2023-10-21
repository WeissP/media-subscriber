use aide::axum::{routing::get, ApiRouter};
use axum::extract::State;
use tracing::instrument;

use crate::store::AppState;

mod channel;
mod search;
mod tag;
pub mod types;

pub fn route() -> ApiRouter<AppState> {
    ApiRouter::new()
        .nest("/tag", tag::route())
        .nest("/channel/:channel_id", channel::route())
        .nest("/search", search::route())
}
