use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::bot::{DocaBot};
use crate::structs::auth::TelegramAuth;
use crate::utils::JsonConfigs;


pub type BotHandler = HashMap<String, ApiRequest>;
pub type UserHandlers = HashMap<String, BotHandler>;

#[derive(PartialEq)]
pub enum ChannelData {
    ReceiveMessage(ReceivedMessage),
    SendMessage(SendMessageRequest),
    Handler(UserHandler),
    PinnedMessage()
}

#[derive(PartialEq)]
pub struct UserHandler {
    pub(crate) bot: String,
    pub(crate) user: UserData,
    pub(crate) handler: BotHandler
}


pub struct AppData {
    pub tx: tokio::sync::mpsc::Sender<ChannelData>,
    pub bots: HashMap<String, Box<dyn DocaBot>>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotButtons {
    pub title: String,
    pub reply: String
}

#[derive(Clone, PartialEq)]
pub struct BotContext {
    pub bot_name: String,
    pub api_url: String
}

#[derive(Clone, PartialEq)]
pub struct  ReceivedMessage {
    pub user: String,
    pub message: String,
    pub ctx: BotContext
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddContactRequest {
    pub messenger: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    pub phone: String,
    pub messenger_id: Option<String>
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub messenger: String,
    pub user: UserData,
    pub message: String,
    pub access_hash: Option<i64>,
    pub buttons: Option<Vec<BotButtons>>,
    pub handlers: Option<BotHandler>
}

#[cfg(test)]
impl JsonConfigs for SendMessageRequest {}

#[cfg(test)]
impl JsonConfigs for AddContactRequest {}