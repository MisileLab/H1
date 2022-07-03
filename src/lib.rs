use std::collections::HashSet;

use async_trait::async_trait;

use serde::Deserialize;

#[async_trait]
pub trait ClientTrait {
    async fn get_stream_informations(&self) -> Result<Option<Vec<StreamProfile>>, reqwest::Error>;
}

#[derive(Deserialize)]
pub struct StreamProfile {
    #[serde(rename = "user_name")]
    pub streamer_name: String,
    pub game_name: String
}

#[derive(Deserialize)]
struct StreamProfileWrapper {
    data: Vec<StreamProfile>
}

#[derive(Clone)]
pub struct Client {
    pub access_token: String,
    pub filter: HashSet<String>
}

#[async_trait]
impl ClientTrait for Client {
    async fn get_stream_informations(&self) -> Result<Option<Vec<StreamProfile>>, reqwest::Error> {
        let result: StreamProfileWrapper = reqwest::Client::new()
        .get("https://api.twitch.tv/helix/streams")
        .bearer_auth(self.access_token.clone())
        .query(&[("first", 100)])
        .query(&[("user_login", self.filter.clone())])
        .send().await?
        .json().await?;

        if result.data.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result.data))
        }
    }
}