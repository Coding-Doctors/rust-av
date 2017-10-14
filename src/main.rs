#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serenity;
extern crate spin;
extern crate toml;

mod handlers;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serenity::prelude::*;
use serenity::model::*;
use serenity::client::CACHE;
use std::env;
use std::collections::HashMap;
use spin::Mutex;
use handlers::Handler;


#[derive(Deserialize)]
struct Config {
    token: String,
    user_roles: Vec<String>,
    log_channel: String,
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = {
        let path = "$HOME/.config/sudobot/config.toml";
        let mut f = File::open(path).unwrap();

        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        let cfg: Config = toml::from_str(&buffer).unwrap();

        Mutex::new(cfg)
    };
}

fn main() {
    let token = &CONFIG.lock().token;

    let mut client = Client::new(token, Handler);

    if let Err(e) = client.start() {
        println!("Error starting client {}", e);
    }
}
