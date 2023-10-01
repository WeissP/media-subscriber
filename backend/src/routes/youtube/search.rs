use super::types::{ChannelID, ChannelInfo, Continuation, VideoInfo};
use crate::{errors::AppError, extractors::Json, Result};
use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    OperationIo,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use invidious::{ClientAsync, ClientAsyncTrait, CommonVideo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use strum::AsRefStr;
use tracing::instrument;
use validator::Validate;

pub fn route() -> ApiRouter {
    ApiRouter::new().api_route("/channel", get(search_channel))
}

#[instrument]
pub async fn search_channel(
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<ChannelInfo>>> {
    let channels = ClientAsync::default()
        .search(Some(&format!("type=channel&q={}", params.query)))
        .await?
        .items
        .into_iter()
        .filter_map(|it| match it {
            invidious::hidden::SearchItem::Channel(ch) => Some(ch.into()),
            _ => None,
        })
        .collect();

    Ok(Json(channels))
}

#[derive(Deserialize, Debug, Validate, JsonSchema)]
pub struct SearchParams {
    /// search keyword
    query: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ChannelResponse {
    author: String,
    channel_id: ChannelID,
}
