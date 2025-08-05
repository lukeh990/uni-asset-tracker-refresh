/*
* main.rs
* Copyright (c) 2025 Luke Harding
* SPDX License Identifier: MIT
*/

pub mod controller;
pub mod model;
pub mod view;

use std::env;

use model::config::{Config, ConfigError};

#[tokio::main]
async fn main() {
    let config_env = env::var("UATR_CONFIG_PATH").unwrap_or(String::from("./config.toml"));

    let view = view::View::init().await;

    let global_config = match Config::load(config_env) {
        Ok(config) => config,
        Err(e) => match e {
            ConfigError::NotFound => Config::blank(),
            _ => {
                view.startup_error(e.into()).await;
                return;
            }
        },
    };
}
