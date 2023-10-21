use super::types::{
    ChannelID, ChannelIDPath, ChannelInfo, Continuation, VideoInfo,
};
use crate::{
    cornucopia::queries::ytb,
    errors::{RespError, Response},
    extractors::Json,
    routes::youtube::types::QueryParam,
    store::AppState,
};
use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    OperationIo,
};
use anyhow::Context;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use axum_valid::Valid;
use invidious::{ClientAsync, ClientAsyncTrait, CommonVideo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use strum::AsRefStr;
use tracing::instrument;
use validator::Validate;

pub fn route() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route("/", get(info))
        .api_route("/videos", get(videos))
}

#[instrument]
pub async fn info(
    State(st): State<AppState>,
    Path(ChannelIDPath { channel_id }): Path<ChannelIDPath>,
) -> Response<Json<ChannelInfo>> {
    let c = st.db().await?;
    let ch = ChannelInfo::get_or_init(channel_id, &c).await?;
    Ok(Json(ch))
}

#[instrument]
pub async fn videos(
    Path(ChannelIDPath { channel_id }): Path<ChannelIDPath>,
    Query(params): Query<VideosParams>,
) -> Response<Json<VideosResponse>> {
    let channel = ClientAsync::default()
        .channel_videos(
            channel_id.as_ref(),
            QueryParam::from(&params.continuation).as_param(),
        )
        .await?;
    let videos_info: Vec<_> =
        channel.videos.into_iter().map(VideoInfo::from).collect();
    Ok(Json(VideosResponse {
        videos_num: videos_info.len(),
        continuation: Continuation(channel.continuation),
        videos_info,
    }))
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct VideosParams {
    continuation: Continuation,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct VideosResponse {
    /// the number of videos returned
    videos_num: usize,
    continuation: Continuation,
    videos_info: Vec<VideoInfo>,
}
