use crate::rover::error::Error;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use zeroize::Zeroizing;

pub mod error;

const BASE_API: &str = "https://registry.rover.link/api";

pub struct RoverClient {
    secret: Zeroizing<String>,
    client: reqwest::Client,
    ratelimit_end_duration: Option<SystemTime>,
}

impl RoverClient {
    pub fn new(client: reqwest::Client, secret: String) -> RoverClient {
        RoverClient {
            secret: Zeroizing::new(secret),
            client,
            ratelimit_end_duration: None,
        }
    }

    pub async fn discord_to_roblox(
        &self,
        guild_id: u64,
        user_id: u64,
    ) -> Result<RobloxInfo, Error> {
        let response = self
            .client
            .get(format!(
                "{BASE_API}/guilds/{guild_id}/discord-to-roblox/{user_id}"
            ))
            .bearer_auth(&*self.secret)
            .send()
            .await?;

        if response.status().is_client_error() || response.status().is_server_error() {
            return Err(Error::RoverError(response.json().await?));
        }

        let info = response.json::<RobloxInfo>().await?;

        Ok(info)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RobloxInfo {
    pub roblox_id: u64,
    pub discord_id: String,
    pub guild_id: String,
    pub cached_username: String,
}
