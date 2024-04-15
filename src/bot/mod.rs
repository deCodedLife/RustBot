pub mod telegram;

use grammers_client::Update;
use grammers_mtsender::InvocationError;
use grammers_session::PackedChat;
use grammers_tl_types::{Serializable};
use serde::{Deserialize, Serialize};
use crate::structs::*;
use crate::structs::api::{AddContactRequest, BotHandler, SendMessageRequest};
use crate::utils;
use crate::utils::JsonConfigs;


#[derive(Default, Serialize, Deserialize)]
pub struct BotAuth {
    pub app_id: i32,
    pub app_hash: String
}
impl JsonConfigs for BotAuth{}


#[derive(Debug)]
pub struct BotChat {
    pub chat_id: Option<i64>,
    pub title: Option<String>,
    pub tg_chat: Option<PackedChat>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct BotContact {
    pub user_id: i64,
    pub username: String
}


pub trait DocaBot {
    async fn new(cfg: BotAuth) -> Self;
    fn add_handler(&mut self, user: String, handler: BotHandler);
    async fn sign_in(&self, data: auth::AuthData) -> utils::Result<()>;
    async fn sign_out(&self);
    async fn get_dialogs(&self) -> utils::Result<Vec<BotChat>>;
    async fn get_contacts(&self) -> utils::Result<Vec<BotContact>>;
    async fn send_message(&self, data: SendMessageRequest) -> utils::Result<()>;
    async fn add_contact(&self, data: AddContactRequest) -> utils::Result<i64>;
    async fn get_updates(&self) -> Result<Option<Update>, InvocationError>;
    async fn handle_message(&self, user: String, message: String) -> utils::Result<()>;
}