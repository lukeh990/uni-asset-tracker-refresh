/*
* model/mod.rs
* Copyright (c) 2025 Luke Harding
* SPDX License Identifier: MIT
*/

use chrono::NaiveDate;

mod asset;
mod datasource;

pub use asset::Asset;
pub use datasource::DataSource;

pub struct IdNamePair {
    id: String,
    name: String,
}

pub struct RemovalInfo {
    removal_date: NaiveDate,
    removal_method: IdNamePair,
}

impl IdNamePair {
    pub fn new<S: Into<String>>(id: S, name: S) -> Self {
        let id = id.into();
        let name = name.into();
        Self { id, name }
    }
}

impl RemovalInfo {
    pub fn new(removal_date: NaiveDate, removal_method: IdNamePair) -> Self {
        Self {
            removal_date,
            removal_method,
        }
    }
}
