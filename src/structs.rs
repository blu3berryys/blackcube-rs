// use std::collections::HashMap;

use poise::serenity_prelude::{ChannelId, GuildId, RoleId};
use reqwest::Client;
use s3::Bucket;
pub use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub bot: Bot,
    pub storage: Storage,
    pub server: Server,
    pub settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub image_types: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    pub application_id: u64,
    pub discord_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    pub url: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket_name: String,
    pub storage_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub guild_id: GuildId,
    pub request_channel_id: ChannelId,
    pub log_channel_id: ChannelId,
    pub command_channel_id: ChannelId,
    pub auth_role_id: RoleId,
}

pub struct Data {
    pub config: Config,
    pub http_client: Client,
    pub bucket: Bucket,
}
