use super::types::{
    ChannelID, ChannelInfo, Continuation, SortOrder, VideoInfo, VideoSource,
    VideoWithSource,
};
use crate::{
    cornucopia::{self, queries::ytb},
    errors::{RespError, Response},
    extractors::Json,
    store::AppState,
};
use aide::{
    axum::{
        routing::{get, post},
        ApiRouter, IntoApiResponse,
    },
    OperationIo,
};
use anyhow::Context;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use futures::StreamExt;
use invidious::{ClientAsync, ClientAsyncTrait, CommonVideo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, time::SystemTime};
use strum::AsRefStr;
use tracing::instrument;
use validator::Validate;
// use axum_extra::extract::Q

pub fn route() -> ApiRouter<AppState> {
    ApiRouter::new().api_route("/:tag/", get(videos))
    // .api_route("/:tag/", post(insert))
}

#[instrument]
pub async fn videos(
    State(st): State<AppState>,
    Path(PathParams { tag }): Path<PathParams>,
    Query(params): Query<VideosParams>,
) -> Response<Json<VideosResult>> {
    let c = st.db().await?;
    // let channels_string = ytb::channels_by_tags()
    //     .bind(&c, &params.tags)
    //     .all()
    //     .await
    //     .context("could not query channels by tags")?;
    // let channels: HashSet<ChannelID> = channels_string
    //     .into_iter()
    //     .flat_map(|s| match s.try_into() {
    //         Ok(x) => Some(x),
    //         Err(e) => {
    //             tracing::error!("invalid channel id is stored: {}", e);
    //             None
    //         }
    //     })
    //     .collect();
    // Ok(Json(SearchResult { channels }))
    todo!()
}

#[derive(Deserialize, Debug, Validate, JsonSchema)]
pub struct VideosParams {
    /// The sort order of all videos of this tag
    sort_order: Option<SortOrder>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct VideosResult {
    /// all videos of this tag sorted by the given order
    videos: Vec<VideoWithSource>,
}

#[instrument]
pub async fn insert(
    State(st): State<AppState>,
    Path(PathParams { tag }): Path<PathParams>,
    Json(InsertParams { channels }): Json<InsertParams>,
) -> Response<()> {
    let c = st.db().await?;
    let channel_id_str: Vec<&str> = channels.iter().map(|s| s.as_ref()).collect();
    // ytb::insert_tag_channels()
    //     .bind(&c, &tag, &channel_id_str)
    //     .await
    //     .context("could not insert channels by tags")?;
    Ok(())
}

#[derive(Deserialize, Debug, Validate, JsonSchema)]
pub struct InsertParams {
    channels: Vec<ChannelID>,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct PathParams {
    /// the tag for a set of channels and/or videos
    pub tag: String,
}
