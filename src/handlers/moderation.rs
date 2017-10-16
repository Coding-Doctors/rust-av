use serenity::prelude::*;
use serenity::model::*;
use serenity::client::CACHE;
use crate::Config;
use std::error::Error;

pub fn ban_handler(_: Context, guild_id: GuildId, user: User, cfg: Config) -> Result<String, _> {
    //This is safe because we are the only ones who hold the lock.
    let cache = CACHE.read().unwrap();

    let mut guild = cache.guild();
    
    //No guild found for the specified guild id.
    if guild.is_none() {
        Err(&format!("No guild found for this guild id."))
    }

    //Safe.
    guild = guild.read().unwrap();
    
    //The ban we want to log.
    let ban_info = {
        let iterator = guild.bans().unwrap().iter_mut();

        for ban in iterator {
            if ban.user == user {
                ban
            } else {
                info!("User {} is not banned from server", user.name, guild.name);
            }
        }
    };
    
    //Format the info about the banned user as a unique combination of name and discriminator.
    //e.g: toor#5207
    let user_discrim = format!("{}{}", user.name, user.discriminator.to_string());

    let reason = ban_info.reason;

    let mut log_msg: String;
    
    //If no reason.
    if reason.is_none() {
        log_msg = format!("User {} was banned.", user_discrim);
    } else {
        log_msg = format!("User {} was banned for reason {}", user.name, reason.unwrap());
    }

    log_msg
}
