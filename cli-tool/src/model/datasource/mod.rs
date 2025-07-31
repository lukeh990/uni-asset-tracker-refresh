/*
* model/datasource/mod.rs
* Copyright (c) 2025 Luke Harding
* SPDX License Identifier: MIT
*/

use tokio::sync::mpsc::{self, error::TryRecvError};

mod command;
mod error;

pub use command::DataSourceCommand;
pub use error::DataSourceError;

pub struct DataSourceConnInfo {
    server: String,
    cookie: String,
}

pub struct DataSource {
    connection_info: DataSourceConnInfo,
    pub command_tx: mpsc::Sender<DataSourceCommand>,
    command_rx: mpsc::Receiver<DataSourceCommand>,
}

impl DataSource {
    pub fn new<S: Into<String>>(server: S, cookie: S) -> Self {
        let server = server.into();
        let cookie = cookie.into();

        // Buffer size. Straight from my ass.
        let (command_tx, command_rx) = mpsc::channel(10);

        Self {
            connection_info: DataSourceConnInfo { server, cookie },
            command_tx,
            command_rx,
        }
    }

    pub async fn start(mut self) -> () {
        println!("DataSource startup...");

        let mut running = true;

        while running {
            match self.command_rx.try_recv() {
                Ok(cmd) => {}
                Err(e) => {
                    if e == TryRecvError::Disconnected {
                        running = false;
                    }
                }
            }
        }

        println!("DataSource exiting...");
    }
}
