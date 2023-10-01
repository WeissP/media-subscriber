use aide::axum::ApiRouter;

mod channel;
mod search;
pub mod types;

pub fn route() -> ApiRouter {
    ApiRouter::new()
        .nest("/channel/:channel_id", channel::route())
        .nest("/search", search::route())
}
