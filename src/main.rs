extern crate serenity;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
struct Config {
    token: String,
    user_roles: Vec<String>,
}

fn main() {
    
}
