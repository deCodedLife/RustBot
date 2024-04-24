use serde::{Deserialize, Serialize};
use crate::utils::JsonConfigs;
use std::collections::HashMap;

#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct TelegramAuth {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) verify_code: String
}


#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct WhatsAppAuth {
    pub(crate) phone_id: String
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum AuthData {
    Telegram(TelegramAuth),
    WhatsApp(WhatsAppAuth)
}


pub type AuthList = HashMap<String, TelegramAuth>;


impl JsonConfigs for TelegramAuth{}
impl JsonConfigs for WhatsAppAuth{}