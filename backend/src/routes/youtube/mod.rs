use aide::axum::ApiRouter;

use crate::store::AppState;

mod channel;
mod search;
mod tag;
pub mod types;

pub fn route() -> ApiRouter<AppState> {
    ApiRouter::new()
        .nest("/channel/:channel_id", channel::route())
        .nest("/search", search::route())
        .nest("/tag", tag::route())
}
