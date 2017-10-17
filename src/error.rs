extern crate std;

use std::sync;
use toml;

pub enum Error {
    Io(std::io::Error),
    TomlError(toml::de::Error),
    GuildError(String),
}
