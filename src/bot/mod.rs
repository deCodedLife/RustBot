pub mod telegram;

use grammers_client::Update;
use grammers_mtsender::InvocationError;
use grammers_session::PackedChat;
use grammers_tl_types::{Serializable};
use grammers_tl_types::types::InputPeerNotifySettings;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::structs::*;
use crate::utils;
use crate::utils::JsonConfigs;

// impl crate::Serializable for InputPeerNotifySettings {
//     fn serialize(&self, buf: crate::serialize::Buffer) {
//         (0u32 | if self.show_previews.is_some() { 1 } else { 0 } | if self.silent.is_some() { 2 } else { 0 } | if self.mute_until.is_some() { 4 } else { 0 } | if self.sound.is_some() { 8 } else { 0 } | if self.stories_muted.is_some() { 64 } else { 0 } | if self.stories_hide_sender.is_some() { 128 } else { 0 } | if self.stories_sound.is_some() { 256 } else { 0 }).serialize(buf);
//         if let Some(ref x) = self.show_previews {
//             x.serialize(buf);
//         }
//         if let Some(ref x) = self.silent {
//             x.serialize(buf);
//         }
//         if let Some(ref x) = self.mute_until {
//             x.serialize(buf);
//         }
//         if let Some(ref x) = self.sound {
//             x.serialize(buf);
//         }
//         if let Some(ref x) = self.stories_muted {
//             x.serialize(buf);
//         }
//         if let Some(ref x) = self.stories_hide_sender {
//             x.serialize(buf);
//         }
//         if let Some(ref x) = self.stories_sound {
//             x.serialize(buf);
//         }
//     }
// }


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
    async fn sign_in(&self, data: auth::AuthData) -> utils::Result<()>;
    async fn sign_out(&self);
    async fn get_dialogs(&self) -> utils::Result<Vec<BotChat>>;
    async fn get_contacts(&self) -> utils::Result<Vec<BotContact>>;
    async fn send_message(&self, chat: &BotChat, message: &str) -> utils::Result<()>;
    async fn get_updates(&self) -> Result<Option<Update>, InvocationError>;
}