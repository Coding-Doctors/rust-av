
pub use super::{Config, get_config};
use serenity::client::CACHE;
use serenity::model::*;
use serenity::prelude::*;

mod moderation;

pub struct Handler {
    pub cfg: Config,
}

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected and serving {} servers.",
                 ready.user.name,
                 ready.guilds.len());
    }

    fn on_guild_ban_addition(&self, _: Context, guild_id: GuildId, user: User) {
        println!("Found a ban!");
        let log_msg = moderation::ban_handler(guild_id, user, &self.cfg).unwrap();

        // Convert the log_channel from a u64 to a ChannelId.
        let log_channel = ChannelId(self.cfg.log_channel);

        if let Err(e) = log_channel.say(log_msg) {
            error!("Error sending log message: {}", e);
        }
    }
}
