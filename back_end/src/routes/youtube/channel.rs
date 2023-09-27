use super::types::{ChannelID, Continuation, VideoInfo};
use crate::{errors::AppError, extractors::Json, Result};
use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    OperationIo,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use axum_valid::Valid;
use invidious::{ClientAsync, ClientAsyncTrait, CommonVideo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use strum::AsRefStr;
use validator::Validate;

pub fn route() -> ApiRouter {
    ApiRouter::new().api_route("/videos", get(videos))
}

pub async fn videos(
    Path(ChannelID { channel_id }): Path<ChannelID>,
    Query(params): Query<VideosParams>,
) -> Result<Json<VideosResponse>> {
    tracing::debug!("<videos> channel-id=[{channel_id}], {:?}", params);
    let channel = ClientAsync::default()
        .channel_videos(
            &channel_id,
            params
                .continuation
                .map(|c| format!("continuation={}", c.0))
                .as_ref()
                .map(String::as_str),
        )
        .await?;
    let videos_info: Vec<_> =
        channel.videos.into_iter().map(VideoInfo::from).collect();
    Ok(Json(VideosResponse {
        videos_num: videos_info.len(),
        continuation: Continuation(channel.continuation.unwrap()),
        videos_info,
    }))
}

#[derive(Deserialize, Debug, Validate, JsonSchema)]
pub struct VideosParams {
    /// A continuation token to get the next chunk of items, If null then the first batch of videos will be responsed
    continuation: Option<Continuation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct VideosResponse {
    /// the number of videos returned
    videos_num: usize,
    continuation: Continuation,
    videos_info: Vec<VideoInfo>,
}
