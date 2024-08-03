use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use crate::bot::{DocaBot, MessagesMap};
use crate::structs::api::{AddContactRequest, BotHandler, SendMessageRequest, UserData};
use crate::structs::wrapper::{ChannelTx};
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
    // pub async fn new(_: BotAuth) -> Self {
    //     todo!()
    // }
}

#[async_trait]
impl DocaBot for WhatsApp {
    fn get_bot_name(self) -> String {
        String::from("whatsapp")
    }

    fn add_handler(&mut self, _: UserData, _: BotHandler) -> utils::Result<()> {
        todo!()
    }

    async fn sign_in(&mut self, _: String, _: AuthData) -> utils::Result<()> {
        todo!()
    }

    async fn sign_out(&self) {
        todo!()
    }

    async fn send_message(&self, _: SendMessageRequest) -> utils::Result<()> {
        todo!()
    }

    async fn add_contact(&self, _: AddContactRequest) -> utils::Result<()> {
        todo!()
    }

    async fn get_dialogs(&self) -> utils::Result<MessagesMap> {
        todo!()
    }

    // async fn custom_handler(&mut self, bot_ctx: BotContext, tx: Sender<ChannelData>) {
    //     todo!()
    // }

    async fn update_profile_status(&self) {
        todo!()
    }

    async fn message_handler(&self, _: Sender<ChannelTx>) {
        todo!()
    }

    async fn handle_message(&self, _: String, _: String) -> utils::Result<()> {
        todo!()
    }

    async fn delete_contacts(&self) {
        todo!()
    }

    fn start_handle(self, _: Sender<ChannelTx>) {
        todo!()
    }

    fn clone_boxed(&self) -> Box<dyn DocaBot + 'static> {
        Box::new(self.clone())
    }
}