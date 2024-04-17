use grammers_client::Update;
use grammers_mtsender::InvocationError;
use serde::{Deserialize, Serialize};
use crate::bot::{BotAuth, BotChat, BotContact, DocaBot};
use crate::structs::api::{AddContactRequest, BotHandler, SendMessageRequest};
use crate::structs::auth::AuthData;

#[derive(PartialEq, Default, Serialize, Deserialize)]
pub struct TelegramBotAuth {
    pub(crate) token: String
}

pub struct TelegramBot {
    pub(crate) api_url: String,
    pub(crate) token: String
}

impl DocaBot for TelegramBot {
    async fn new(cfg: BotAuth) -> Self {
        todo!()
    }

    fn add_handler(&mut self, user: String, handler: BotHandler) {
        todo!()
    }

    async fn sign_in(&self, data: AuthData) -> crate::utils::Result<()> {
        todo!()
    }

    async fn sign_out(&self) {
        todo!()
    }

    async fn get_dialogs(&self) -> crate::utils::Result<Vec<BotChat>> {
        todo!()
    }

    async fn get_contacts(&self) -> crate::utils::Result<Vec<BotContact>> {
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

    async fn handle_message(&self, user: String, message: String) -> crate::utils::Result<()> {
        todo!()
    }
}