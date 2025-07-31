/*
* main.rs
* Copyright (c) 2025 Luke Harding
* SPDX License Identifier: MIT
*/

use model::DataSource;
use tokio::spawn;

pub mod controller;
pub mod model;
pub mod view;

#[tokio::main]
async fn main() {
    println!("Starting model datasource!");

    // TODO: Replace with proper values.
    let datasource = DataSource::new("https://example.com", "TESTCOOKIE");

    spawn(datasource.start());
}
