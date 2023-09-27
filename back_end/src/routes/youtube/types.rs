use aide::OperationIo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
pub struct ChannelID {
    /// The youtube channel ID.
    pub channel_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
/// A continuation token to get the next chunk of items.
pub struct Continuation(pub String);

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct VideoInfo {
    title: String,
    /// published unix timestamp in seconds
    published: u64,
    /// duration seconds
    length: u32,
    id: String,
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
            ..
        }: invidious::CommonVideo,
    ) -> Self {
        Self {
            title,
            published,
            length,
            id,
            description_html,
        }
    }
}
