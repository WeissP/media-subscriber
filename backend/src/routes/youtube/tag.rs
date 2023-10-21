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
use futures::{
    future::{join, try_join},
    select,
    stream::{self, select_all, FuturesUnordered},
    StreamExt,
};
use invidious::{ClientAsync, ClientAsyncTrait, CommonVideo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, time::SystemTime};
use strum::AsRefStr;
use tracing::instrument;
use validator::Validate;
// use axum_extra::extract::Q

pub fn route() -> ApiRouter<AppState> {
    ApiRouter::new().api_route("/:tag", get(videos))
    // .api_route("/:tag/", post(insert))
}

#[instrument]
pub async fn videos(
    State(st): State<AppState>,
    Path(PathParams { tag }): Path<PathParams>,
    Query(params): Query<VideosParams>,
) -> Response<Json<VideosResult>> {
    let c = st.db().await?;
    let channels_string = ytb::channels_by_tag()
        .bind(&c, &tag)
        .all()
        .await
        .context("could not query channels by tags")?;
    let client = ClientAsync::default();
    let mut channel_task = channels_string
        .into_iter()
        .flat_map(|s| match s.try_into() {
            Ok(x) => Some(x),
            Err(e) => {
                tracing::error!("invalid channel id is stored: {}", e);
                None
            }
        })
        .map(|id| {
            VideoWithSource::of_channel(&client, id, Continuation::default())
        })
        .collect::<FuturesUnordered<_>>();
    let mut videos_response = Vec::new();
    while let Some(res) = channel_task.next().await {
        match res {
            Ok(videos) => videos_response.extend(videos),
            Err(e) => {
                tracing::error!("could not fetch videos: {:?}", e);
            }
        }
    }
    let recommand_videos = VideoWithSource::of_recommandation(tag, &c).await?;
    videos_response.extend(recommand_videos);
    Ok(Json(VideosResult {
        videos: videos_response,
    }))
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
