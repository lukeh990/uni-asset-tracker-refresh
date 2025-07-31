/*
* model/datasource/command.rs
* Copyright (c) 2025 Luke Harding
* SPDX License Identifier: MIT
*/

use super::{DataSourceConnInfo, DataSourceError};
use crate::model::Asset;
use std::result;
use tokio::sync::oneshot;

type Result<T> = result::Result<T, DataSourceError>;
type Reply<T> = oneshot::Sender<Result<T>>;

pub enum DataSourceCommand {
    GetAsset(String, Reply<Asset>),
}

impl DataSourceCommand {
    fn get_asset(state: DataSourceConnInfo, id: String) -> Result<Asset> {
        Ok(())
    }

    pub async fn parse(self, state: DataSourceConnInfo) {
        match self {
            Self::GetAsset(id, reply) => {
                reply.send(Self::get_asset(state, id));
            }
        }
    }
}
