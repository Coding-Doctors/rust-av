extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serenity;
extern crate spin;
extern crate toml;
extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_error;

mod handlers;
mod error;
mod commands;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::error::Error;
use serenity::prelude::*;
use serenity::model::*;
use serenity::framework::StandardFramework;
use handlers::Handler;

#[derive(Deserialize)]
pub struct Config {
    token: String,
    mod_id: u64,
    admin_id: u64,
    log_channel: u64,
}

pub fn get_config() -> Config {
    let home_dir = env::home_dir().unwrap();
    let path = home_dir.join(".config").join("sudobot").join("config.toml");

    let mut f = File::open(path).unwrap();
    let mut buf = String::new();

    f.read_to_string(&mut buf).unwrap();

    toml::from_str(&buf).unwrap()
}

fn main() {
    match env_logger::init() {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to initialize env_logger. Reason: {}", e.cause().unwrap());
        },
    }

    let cfg = get_config();
        
    let token = cfg.token.clone();
    
    let handler = Handler {
        cfg: cfg,
    };

    let mut client = Client::new(&token, handler);

    client.with_framework(
        StandardFramework::new()
        .configure(|c| c
            .prefix("sudo")
            .on_mention(true))
        .before(|_ctx, msg, command_name| {
            info!("Got command {} by user {}", command_name, msg.author.name);
            true
        })
        .group("Moderation", |g| g
            .command("ban", |c| c
                .exec(commands::ban)
                .desc("Bans a user from a guild."))
            .command("kick", |c| c
                .exec(commands::kick)
                .desc("Kicks a user from a guild.")))
        );

    if let Err(e) = client.start() {
        error!("Client error: {:?}", e);
    }
}
