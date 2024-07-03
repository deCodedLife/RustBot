use async_trait::async_trait;
use grammers_mtsender::InvocationError;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use grammers_client::Update;
use crate::bot::{BotAuth, DocaBot, MessagesMap};
use crate::structs::api::{AddContactRequest, BotContext, BotHandler, ChannelData, SendMessageRequest, UserData};
use crate::structs::auth::AuthData;
use crate::utils;
use crate::utils::JsonConfigs;

#[derive(PartialEq, Default, Serialize, Deserialize)]
pub struct WhatsappAuth {
    phone_id: String,
    account_id: String,
}

impl JsonConfigs for WhatsappAuth {}

#[derive(Clone)]
pub struct WhatsApp {
    pub token: String,
}

impl WhatsApp {
    pub async fn new(cfg: BotAuth) -> Self {
        todo!()
    }
}

#[async_trait]
impl DocaBot for WhatsApp {
    fn get_bot_name(self) -> String {
        String::from("whatsapp")
    }

    fn add_handler(&mut self, user: UserData, handler: BotHandler) {
        todo!()
    }

    async fn sign_in(&mut self, bot_name: String, data: AuthData) -> utils::Result<()> {
        todo!()
    }

    async fn sign_out(&self) {
        todo!()
    }

    async fn send_message(&self, data: SendMessageRequest) -> utils::Result<()> {
        todo!()
    }

    async fn add_contact(&self, data: AddContactRequest) -> utils::Result<i64> {
        todo!()
    }

    async fn get_dialogs(&self) -> utils::Result<MessagesMap> {
        todo!()
    }

    async fn get_updates(&self) -> Result<Option<Update>, InvocationError> {
        todo!()
    }

    async fn custom_handler(&mut self, bot_ctx: BotContext, tx: Sender<ChannelData>) {
        todo!()
    }

    async fn message_handler(&self, bot_ctx: BotContext, tx: tokio::sync::mpsc::Sender<ChannelData>) -> utils::Result<()> {
        todo!()
    }

    async fn handle_message(&mut self, user: String, ctx: BotContext, message: String) -> utils::Result<()> {
        todo!()
    }

    async fn delete_contacts(&self) {
        todo!()
    }

    fn clone_boxed(&self) -> Box<dyn DocaBot + 'static> {
        Box::new(self.clone())
    }
}