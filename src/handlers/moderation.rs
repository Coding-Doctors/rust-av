use serenity::prelude::*;
use serenity::model::*;
use serenity::client::CACHE;

pub fn ban_handler(&self, _: Context, guild_id: GuildId, user: User) {
    //This is safe because we are the only ones who hold the lock.
    let cache = CACHE.read().unwrap();

    let guild = match cache.guild(guild_id) {
        Some(g) => g,
        None => None
    };
    
    //If no guild was found...
    if guild.is_none() {
        error!("No guild found for GuildId {}", guild_id);
    }

    //Safe to assume guild holds a Guild object now because otherwise this code would be
    //unreachable.
    let log_chan = self.cfg.log_channel;

    for value in guild.channels.values() {
        //No other threads should have acquired this lock, so we assume it isn't poisoned.
        let chan = value.read().unwrap()

        chan.is_err() {
            error!("Error acquiring RwLock on channel. Caused by: {}", chan.unwrap_err());
        }

        let user_ban = {
            for ban in guild.bans()
        }
    }
}
