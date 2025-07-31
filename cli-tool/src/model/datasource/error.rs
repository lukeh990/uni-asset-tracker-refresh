/*
* model/datasource/error.rs
* Copyright (c) 2025 Luke Harding
* SPDX License Identifier: MIT
*/

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataSourceError {
    #[error("Unknown data source error!")]
    Unknown,
}
