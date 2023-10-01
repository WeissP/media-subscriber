use aide::OperationIo;
use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
};
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "String")]
pub struct ChannelID {
    /// The youtube channel ID (must be 24 ASCII characters), e.g., UCjuNibFJ21MiSNpu8LZyV4w
    pub channel_id: [u8; 24],
}

// pub struct ChannelID(pub [u8; 24]);

impl AsRef<str> for ChannelID {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(&self.channel_id)
            .expect("channel id is not valid utf-8")
    }
}

impl TryFrom<String> for ChannelID {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            channel_id: value.as_bytes().try_into().map_err(|_| {
                format!(
                    "channel id should have exact 24 characters, received: {}",
                    value
                )
            })?,
        })
    }
}

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
    /// description in HTML form
    description_html: String,
}

impl From<invidious::CommonChannel> for ChannelInfo {
    fn from(value: invidious::CommonChannel) -> Self {
        Self {
            name: value.name,
            id: value.id.try_into().unwrap_or_else(|e| {
                panic!("invidious has an invalid channel id: {e}")
            }),
            description_html: value.description_html,
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

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use super::*;
    #[test]
    fn schema_channel_id() {
        let schema = schema_for!(ChannelID);
        panic!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
