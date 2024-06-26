pub mod telegram;
pub mod whatsapp;

use async_trait::async_trait;
use grammers_client::Update;
use grammers_mtsender::InvocationError;
use grammers_session::PackedChat;
use grammers_tl_types::{Serializable};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::bot::telegram::{TelegramAuth};
use crate::bot::whatsapp::{WhatsappAuth};
use crate::structs::*;
use crate::structs::api::{AddContactRequest, BotContext, BotHandler, ChannelData, SendMessageRequest, UserData};
use crate::utils;
use crate::utils::JsonConfigs;


#[derive(PartialEq, Serialize, Deserialize)]
pub enum BotAuth {
    TelegramAuth(TelegramAuth),
    WhatsappAuth(WhatsappAuth),
}

#[derive(Debug)]
pub struct BotChat {
    pub chat_id: Option<i64>,
    pub title: Option<String>,
    pub tg_chat: Option<PackedChat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BotContact {
    pub user_id: i64,
    pub username: String,
}

#[async_trait]
pub trait DocaBot: Send + Sync {
    fn get_bot_name(self) -> String;
    fn add_handler(&mut self, user: UserData, handler: BotHandler);
    fn start_dialog(&mut self, user: AddContactRequest) -> Value {
        Value::Null
    }
    async fn sign_in(&self, bot_name: String, data: auth::AuthData) -> utils::Result<()>;
    async fn sign_out(&self);
    async fn send_message(&self, data: SendMessageRequest) -> utils::Result<()>;
    async fn add_contact(&self, data: AddContactRequest) -> utils::Result<i64>;
    async fn get_updates(&self) -> Result<Option<Update>, InvocationError>;
    async fn message_handler(&self, bot_ctx: BotContext, tx: tokio::sync::mpsc::Sender<ChannelData>) -> utils::Result<()>;
    async fn handle_message(&mut self, user: String, ctx: BotContext, message: String) -> utils::Result<()>;
    async fn delete_contacts(&self);
    fn clone_boxed(&self) -> Box<dyn DocaBot>;
}

#[async_trait]
impl Clone for Box<dyn DocaBot> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}