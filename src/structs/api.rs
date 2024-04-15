use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::bot::telegram::TelegramBot;
use crate::utils::JsonConfigs;


pub type BotHandler = HashMap<String, ApiRequest>;
pub type UserHandlers = HashMap<String, BotHandler>;

#[derive(PartialEq)]
pub enum ChannelData {
    Message(ReceivedMessage),
    Handler(UserHandler)
}

#[derive(PartialEq)]
pub struct UserHandler {
    pub(crate) user: String,
    pub(crate) handler: BotHandler
}

#[derive(Clone)]
pub struct AppData {
    pub(crate) bot: TelegramBot,
    pub(crate) tx: tokio::sync::mpsc::Sender<ChannelData>
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotButtons {
    title: String,
    reply: String
}

#[derive(Clone, PartialEq)]
pub struct  ReceivedMessage {
    pub user: String,
    pub message: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiRequest {
    #[serde(skip_serializing)]
    pub api_url: String,
    pub object: String,
    pub command: String,
    pub data: Value
}

#[derive(PartialEq)]
pub enum BotRequestType {
    RequestContact(AddContactRequest),
    RequestMessage(SendMessageRequest)
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddContactRequest {
    pub first_name: String,
    pub last_name: String,
    pub phone: String
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub messenger: String,
    pub user: String,
    pub message: String,
    pub buttons: Option<Vec<BotButtons>>,
    pub handlers: Option<BotHandler>
}

#[cfg(test)]
impl JsonConfigs for SendMessageRequest {}

#[cfg(test)]
impl JsonConfigs for AddContactRequest {}