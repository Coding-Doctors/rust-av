extern crate std;

use std::sync;
use toml;

#[derive(Debug, Error)]
pub enum Error {
    Io(std::io::Error),
    TomlError(toml::de::Error),
}
