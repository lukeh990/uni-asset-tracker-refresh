/*
* view/mod.rs
* Copyright (c) 2025 Luke Harding
* SPDX License Identifier: MIT
*/

use crate::model::config::ConfigError;
use cursive::views::TextView;
use cursive::{CursiveRunnable, CursiveRunner};
use tokio::{
    spawn,
    sync::mpsc::{self, error::TryRecvError},
};

#[derive(thiserror::Error, Debug)]
pub enum ViewError {
    #[error("Failed to read config!")]
    Config(#[from] ConfigError),
}

pub enum ViewCommand {
    FatalError(ViewError),
}

pub struct View {
    view_cmd_tx: mpsc::Sender<ViewCommand>,
}

fn parse_cmd(siv: &mut CursiveRunner<CursiveRunnable>, cmd: ViewCommand) {
    match cmd {
        ViewCommand::FatalError(e) => {
            siv.add_global_callback('q', |s| s.quit());

            siv.add_layer(TextView::new(format!(
        "!!!ERROR!!!\n-------------------------\n{e}\n-------------------------\nPress q to exit"
    )));

            siv.refresh();
        }
    }
}

impl View {
    pub async fn init() -> Self {
        let (tx, mut rx) = mpsc::channel(10);

        let view = Self { view_cmd_tx: tx };

        spawn(async move {
            let siv = cursive::default();
            let mut siv = siv.into_runner();

            siv.refresh();
            loop {
                siv.step();

                if !siv.is_running() {
                    break;
                }

                match rx.try_recv() {
                    Ok(cmd) => parse_cmd(&mut siv, cmd),
                    Err(e) => match e {
                        TryRecvError::Disconnected => {
                            //break;
                            continue;
                        }
                        _ => {}
                    },
                }
            }
        });

        view
    }

    pub async fn startup_error(self, e: ViewError) {
        self.view_cmd_tx.send(ViewCommand::FatalError(e)).await;
    }
}
