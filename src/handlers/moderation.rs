use serenity::prelude::*;
use serenity::model::*;
use serenity::client::CACHE;
use super::Config;

pub fn ban_handler(_: Context, guild_id: GuildId, user: User) {
    //This is safe because we are the only ones who hold the lock.
    let cache = CACHE.read().unwrap();

    let guild = match cache.guild(guild_id).unwrap().read() {
        Some(g) => g,
        None => None
    };
    
    //If no guild was found...
    if guild.is_none() {
        error!("No guild found for GuildId {}", guild_id);
    }

    //Safe to assume guild holds a Guild object now because otherwise this code would be
    //unreachable.
    let log_chan = cfg.log_channel;

    let channel = {
        //Assume no other threads have accessed this lock and hence there is no risk of it being
        //poisoned. This is a safe assumption because our program only uses one thread.
        for chan in guild.channels.values().read().unwrap() {
            if chan.name == log_chan {
                chan
            } else {
                info!("No log channel as configured found on this server");
            }
        }
    };

    let ban_info = {
        for ban in guild.bans() {
            if ban.user == user {
                ban
            } else {
                let user_discrim = format!("{}{}", user.name, user.discriminator.to_string());
                info!("No ban found for user {} on server {}({})", user_discrim, guild.name, guild.id);
            }
        }
    };
    
    //Format the info about the banned user as a unique combination of name and discriminator.
    //e.g: toor#5207
    let user_discrim = format!("{}{}", user.name, user.discriminator.to_string());

    let reason = match ban_info.reason {
        Some(r) => r,
        None => None
    };

    if reason.is_none() {
        let log_msg = format!("User {} was banned. No reason given.", user_discrim);
        if let Err(e) = channel.id.say(&log_msg) {
            error!("Error sending log message: {}", e);
        }
    }

    let log_msg = format!("User {} was banned for reason: {}", user_discrim, reason);

    if let Err(e) = channel.id.say(&log_msg) {
        error!("Error sending log message: {}", e);
    }
}
