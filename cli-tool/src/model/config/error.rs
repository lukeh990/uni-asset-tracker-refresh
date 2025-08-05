/*
* model/config/error.rs
* Copyright (c) 2025 Luke Harding
* SPDX License Identifier: MIT
*/

use std::io;
use thiserror::Error;
use toml::de;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error reading config!")]
    IO(#[from] io::Error),
    #[error("Failed to deserialize config!")]
    Deserialize(#[from] de::Error),
    #[error("No config file found!")]
    NotFound,
}
