use async_trait::async_trait;
use grammers_mtsender::InvocationError;
use serde::{Deserialize, Serialize};
use grammers_client::Update;
use crate::bot::{BotAuth, DocaBot};
use crate::structs::api::{AddContactRequest, BotHandler, ChannelData, SendMessageRequest, UserData};
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
    fn add_handler(&mut self, user: UserData, handler: BotHandler) {
        todo!()
    }

    async fn sign_in(&self, data: AuthData) -> crate::utils::Result<()> {
        todo!()
    }

    async fn sign_out(&self) {
        todo!()
    }

    async fn send_message(&self, data: SendMessageRequest) -> crate::utils::Result<()> {
        todo!()
    }

    async fn add_contact(&self, data: AddContactRequest) -> crate::utils::Result<i64> {
        todo!()
    }

    async fn get_updates(&self) -> Result<Option<Update>, InvocationError> {
        todo!()
    }

    async fn message_handler(&self, tx: tokio::sync::mpsc::Sender<ChannelData>) -> utils::Result<()> {
        todo!()
    }

    fn get_bot_name(self) -> String {
        String::from("whatsapp")
    }

    async fn handle_message(&self, user: String, message: String) -> crate::utils::Result<()> {
        todo!()
    }

    fn clone_boxed(&self) -> Box<dyn DocaBot + 'static> {
        Box::new(self.clone())
    }
}