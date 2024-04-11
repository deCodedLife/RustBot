use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::utils::JsonConfigs;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BotButtons {
    title: String,
    reply: String
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ApiRequest {
    object: String,
    command: String,
    data: Value
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BotRequest {
    pub messenger: String,
    pub user_id: String,
    pub message: String,
    pub buttons: Option<Vec<BotButtons>>,
    pub handlers: Option<HashMap<String, ApiRequest>>
}

#[cfg(test)]
impl JsonConfigs for BotRequest {}