use aide::axum::ApiRouter;

mod channel;
mod types;

pub fn route() -> ApiRouter {
    ApiRouter::new().nest("/channel/:channel_id", channel::route())
}
