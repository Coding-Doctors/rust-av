extern crate serenity;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serenity::prelude::*;
use serenity::model::*;
use serenity::client::CACHE;
use std::env;
use std::collections::HashMap;
use spin::Mutex;

#[derive(Deserialize)]
struct Config {
    token: String,
    user_roles: Vec<String>,
    log_channel: String,
}

static CONFIG: Mutex<Config> = Mutex::new(Config);

struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, _: Context, ready: Ready) {
        println!("{} is connected and serving {} servers.", ready.user.name, ready.guilds.len());
    }

    fn on_guild_ban_addition(&self, _: Context, guild_id: GuildId, user: User) {
        let cache = CACHE.read().unwrap();

        if let Some(g) = cache.guild(guild_id) {
            //Grab a Guild object to play with.
            let guild = g.read().unwrap();

            let log_channel = CONFIG.lock().log_channel;

            let channels = guild.channels;

            for value in channels.values() {
                let mut name = value.read().unwrap().name;

                l
            }
        }
    }
}


fn main() {
    
}
