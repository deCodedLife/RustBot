use crate::structs::api::{AddContactRequest, SendMessageRequest, TelegramMessage};

#[derive(PartialEq, Clone)]
pub enum ChannelData {
    ReceiveMessage(TelegramMessage),
    SendMessage(SendMessageRequest),
    // Handler(UserHandler),
    AddContact(AddContactRequest)
}

#[derive(PartialEq, Clone)]
pub struct  ChannelTx {
    pub data: ChannelData,
    pub bot_name: String
}