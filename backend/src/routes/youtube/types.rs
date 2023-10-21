use crate::{cornucopia::queries::ytb, utils::fixed_str};
use aide::OperationIo;
use anyhow::Context;
use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
};
use cornucopia_async::GenericClient;
use invidious::CommonThumbnail;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct QueryParam(String);

impl QueryParam {
    pub fn insert(&mut self, p: impl Into<Self>) -> () {
        if let Some(p) = p.into().as_param() {
            self.0.push_str(&format!("&{p}"))
        }
    }

    pub fn as_param(&self) -> Option<&str> {
        if self.0.is_empty() {
            None
        } else {
            Some(&self.0)
        }
    }
}

fixed_str!(
    ChannelID,
    24,
    "youtube channel ID",
    "The youtube channel ID (must be 24 ASCII characters), e.g., UCjuNibFJ21MiSNpu8LZyV4w"
);

#[derive(Deserialize, Debug, JsonSchema)]
pub struct ChannelIDPath {
    pub channel_id: ChannelID,
}

fixed_str!(
    VideoID,
    11,
    "youtube video ID",
    "The youtube video ID (must be 11 ASCII characters), e.g., lOwjw1Ja83Y"
);

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
/// A continuation token to get the next chunk of items.
/// If it is not given in paramter or it is empty, then the first chunk of items will be responsed.
/// If it is null in response, then it means all items are returned.
pub struct Continuation(pub Option<String>);

impl<'a> From<&'a Continuation> for QueryParam {
    fn from(value: &'a Continuation) -> Self {
        match &value.0 {
            Some(c) => QueryParam(format!("Continuation={c}")),
            None => QueryParam::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ChannelInfo {
    name: String,
    id: ChannelID,
    /// the official channel description in HTML form
    description_html: String,
    /// introduction to the channel
    introduction: Option<String>,
}

impl ChannelInfo {
    async fn update_db_cache(
        &self,
        conn: &impl GenericClient,
    ) -> anyhow::Result<()> {
        ytb::update_channel_cache()
            .bind(conn, &self.name, &self.description_html, &self.id.as_ref())
            .await?;
        Ok(())
    }

    pub async fn fetch(id: &ChannelID) -> anyhow::Result<Self> {
        let resp: invidious::CommonChannel = todo!();
        Ok(resp.into())
    }

    pub async fn get_or_init(
        id: ChannelID,
        conn: &impl GenericClient,
    ) -> anyhow::Result<Self> {
        let channel = id.as_ref();
        let res = ytb::channel_info()
            .bind(conn, &channel)
            .opt()
            .await
            .context("could not get channel info")?;
        match res {
            Some(ytb::ChannelInfo {
                channel: _,
                channel_name: Some(name),
                description: Some(description_html),
                introduction,
            }) => Ok(Self {
                name,
                id,
                description_html,
                introduction,
            }),
            r => {
                if r.is_none() {
                    ytb::insert_channel().bind(conn, &channel).await?;
                }
                let new = Self::fetch(&id).await?;
                new.update_db_cache(conn).await?;
                Ok(new)
            }
        }
    }
}

impl From<invidious::CommonChannel> for ChannelInfo {
    fn from(value: invidious::CommonChannel) -> Self {
        Self {
            name: value.name,
            id: value.id.try_into().unwrap_or_else(|e| {
                panic!("invidious has an invalid channel id: {e}")
            }),
            description_html: value.description_html,
            introduction: None,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub enum VideoSource {
    #[default]
    /// Unknown source
    Unknown,
    /// the video comes from a channel
    Channel {
        /// the channel ID of the video
        channel_id: ChannelID,
    },
    /// the video comes from recommandation
    Recommandation,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub enum SortOrder {
    #[default]
    /// (Default order) Display recommended videos first in updated order, followed by videos from channels in published order.
    RecommandationChannels,
    /// Display videos from channels first in published order, followed by recommended videos in updated order.
    ChannelsRecommandation,
    /// Sort videos by combining both sources: if it is from channels, use its published date; if it is from recommendations, use its updated date.
    Combined,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct Thumbnail {
    #[serde(default)]
    pub quality: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
}

impl From<CommonThumbnail> for Thumbnail {
    fn from(
        CommonThumbnail {
            quality,
            url,
            width,
            height,
        }: CommonThumbnail,
    ) -> Self {
        Self {
            quality,
            url,
            width,
            height,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct VideoInfo {
    title: String,
    /// published unix timestamp in seconds
    published: u64,
    /// duration seconds
    length: u32,
    id: VideoID,
    thumbnails: Vec<Thumbnail>,
    /// description in HTML form
    description_html: String,
}

impl From<invidious::CommonVideo> for VideoInfo {
    fn from(
        invidious::CommonVideo {
            title,
            published,
            length,
            id,
            description_html,
            thumbnails,
            ..
        }: invidious::CommonVideo,
    ) -> Self {
        Self {
            title,
            published,
            length,
            id: id.try_into().unwrap_or_else(|e| {
                panic!("invidious has an invalid video id: {e}")
            }),
            description_html,
            thumbnails: thumbnails.into_iter().map(Thumbnail::from).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
/// Video information with its source
pub struct VideoWithSource {
    video_info: VideoInfo,
    source: VideoSource,
}
