use poise::serenity_prelude::{client::Context, Member, PartialMember, User};

use crate::structs::Data;

pub trait HasAuth {
    async fn has_auth(&self, ctx: &Context, data: &Data) -> anyhow::Result<bool>;
}

impl HasAuth for PartialMember {
    async fn has_auth(&self, _ctx: &Context, data: &Data) -> anyhow::Result<bool> {
        let config = &data.config;
        Ok(self.roles.contains(&config.server.auth_role_id))
    }
}
impl HasAuth for Member {
    async fn has_auth(&self, _ctx: &Context, data: &Data) -> anyhow::Result<bool> {
        let config = &data.config;
        Ok(self.roles.contains(&config.server.auth_role_id))
    }
}
impl HasAuth for User {
    async fn has_auth(&self, ctx: &Context, data: &Data) -> anyhow::Result<bool> {
        let config = &data.config;
        Ok(self
            .has_role(
                &ctx.http,
                config.server.guild_id,
                config.server.auth_role_id,
            )
            .await?)
    }
}
