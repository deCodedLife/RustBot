pub mod telegram;
pub mod whatsapp;

use std::collections::HashMap;
use async_trait::async_trait;
// use grammers_session::PackedChat;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use crate::bot::telegram::{TelegramAuth};
use crate::bot::whatsapp::{WhatsappAuth};
use crate::structs::*;
use crate::structs::api::{AddContactRequest, BotHandler, SendMessageRequest, TelegramMessage, UserData};
use crate::structs::wrapper::ChannelTx;
use crate::utils;

#[derive(PartialEq, Serialize, Deserialize)]
pub enum BotAuth {
    TelegramAuth(TelegramAuth),
    WhatsappAuth(WhatsappAuth),
}

type MessagesMap = HashMap<String, TelegramMessage>;

// #[derive(Debug, Clone)]
// pub struct BotChat {
//     pub chat_id: Option<i64>,
//     pub title: Option<String>,
//     pub tg_chat: Option<PackedChat>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct BotContact {
    pub user_id: i64,
    pub username: String,
}

#[async_trait]
pub trait DocaBot: Send + Sync {
    fn get_bot_name(self) -> String;
    fn add_handler(&mut self, user: UserData, handler: BotHandler) -> utils::Result<()>;
    async fn sign_in(&mut self, bot_name: String, data: auth::AuthData) -> utils::Result<()>;
    async fn sign_out(&self);
    async fn send_message(&self, data: SendMessageRequest) -> utils::Result<()>;
    async fn add_contact(&self, data: AddContactRequest) -> utils::Result<()>;
    async fn get_dialogs(&self) -> utils::Result<crate::bot::MessagesMap>;

    async fn update_profile_status(&self);
    // async fn custom_handler(&mut self, bot_ctx: BotContext, tx: tokio::sync::mpsc::Sender<ChannelData>);
    async fn message_handler(&self, tx: Sender<ChannelTx>);
    async fn handle_message(&self, user: String, message: String) -> utils::Result<()>;
    async fn delete_contacts(&self);

    fn start_handle(self, tx: Sender<ChannelTx>);
    fn clone_boxed(&self) -> Box<dyn DocaBot>;
}

#[async_trait]
impl Clone for Box<dyn DocaBot> {
    fn clone(&self) -> Self {
        self.clone_boxed()
    }
}