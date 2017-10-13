extern crate serenity;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use serde_json::error::Category;
use std::path::Path;
use std::fs::File;

#[derive(Deserialize)]
struct Config {
    token: String,
    user_roles: Vec<String>,
}

fn main() {
    
}

pub fn retrieve_config() -> Result<Config, Option<(usize, usize)>> {
    let mut path: Path = "$HOME/.con"
}
