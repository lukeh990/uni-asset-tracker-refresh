/*
* model/config/mod.rs
* Copyright (c) 2025 Luke Harding
* SPDX License Identifier: MIT
*/

mod error;

pub use error::ConfigError;
use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, path::Path, result};

type Result<T> = result::Result<T, ConfigError>;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub url: String,
    pub cookie: String,
}

impl ServerConfig {
    fn blank() -> Self {
        Self {
            url: String::from("!TEMP!"),
            cookie: String::from("!TEMP!"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn blank() -> Self {
        Self {
            server: ServerConfig::blank(),
        }
    }

    pub fn load<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let file_path = file_path.as_ref();
        Ok(ConfigFile::load(file_path)?.config)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigFile {
    version: u32,
    config: Config,
}

impl ConfigFile {
    fn load(file_path: &Path) -> Result<Self> {
        if file_path.exists() {
            let config_str = read_to_string(file_path)?;
            Ok(toml::from_str(&config_str)?)
        } else {
            Err(ConfigError::NotFound)
        }
    }
}
