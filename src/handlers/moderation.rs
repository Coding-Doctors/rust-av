use serenity::prelude::*;
use serenity::model::*;
use serenity::client::CACHE;
use Config;
use std::error::Error;

pub fn ban_handler(_: Context, guild_id: GuildId, user: User, cfg: Config) -> Result<String, _> {
    //This is safe because we are the only ones who hold the lock.
    let cache = CACHE.read().unwrap();

    let mut guild = cache.guild(guild_id);
    
    //No guild found for the specified guild id.
    if guild.is_none() {
        Err(&format!("No guild found for this guild id."))
    }

    //Safe.
    guild = guild.read().unwrap();

    let log_msg: String;
    //The ban we want to log.
    match guild.bans().unwrap().iter().find(|b| b.user.id == user.id) {
        Some(b) => {
            let reason = &b.reason;
            //If no reason.
            if reason.is_none() {
                log_msg = format!("User {} was banned.", user.mention());
            } else {
                log_msg = format!("User {} was banned for reason {}", user.name, reason.clone().unwrap());
            }
            log_msg
            },
        None => {
            log_msg = format!("User {} is not banned from server", user.name);
            log_msg
        }
    }
}
