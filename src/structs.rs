use image::ImageFormat;
use poise::serenity_prelude::{ChannelId, GuildId, RoleId};
pub use serde::{Deserialize, Serialize};
use reqwest::Client;
use s3::Bucket;

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
    pub content_types: ContentTypes,
    pub http_client: Client,
    pub bucket: Bucket,
}

pub struct ContentTypes {
    pub valid_content_types: Vec<ImageFormat>,
    pub concatenated_content_types: String
}
