use serenity::prelude::*;
use serenity::model::*;
use serenity::client::CACHE;
pub use super::Config;
mod moderation;

pub struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        println!(
            "{} is connected and serving {} servers.",
            ready.user.name,
            ready.guilds.len()
        );
    }

    fn on_guild_ban_addition(&self, _: Context, guild_id: GuildId, user: User) {}
}
