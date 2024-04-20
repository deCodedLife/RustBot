use std::pin::pin;
use async_trait::async_trait;
use futures_util::future::{Either, select};
use grammers_client::{Client, Config, InputMessage, SignInError, Update};
use grammers_mtsender::InvocationError;
use grammers_session::{PackedChat, PackedType, Session};
use grammers_tl_types::enums::{InputContact};
use grammers_tl_types::types::{InputPhoneContact};
use serde::{Deserialize, Serialize};

use crate::bot::{BotAuth, BotChat, BotContact, DocaBot};
use crate::structs::auth::AuthData;
use crate::{SESSION_FILE, utils};
use crate::structs::api::{AddContactRequest, SendMessageRequest, BotHandler, UserHandlers, ChannelData, ReceivedMessage, UserData};
use crate::utils::JsonConfigs;

#[derive(PartialEq, Default, Serialize, Deserialize)]
pub struct TelegramAuth {
    pub app_id: i32,
    pub app_hash: String,
}

impl JsonConfigs for TelegramAuth {}

#[derive(Clone)]
pub struct Telegram {
    pub client: Client,
    pub handlers: UserHandlers,
}

impl Telegram {
    pub async fn get_contacts(&self) -> utils::Result<Vec<BotContact>> {
        let contacts = Vec::<BotContact>::new();
        let _ = self.client.invoke(&grammers_tl_types::functions::contacts::GetContacts { hash: 0 }).await?;
        Ok(contacts)
    }

    async fn get_dialogs(&self) -> utils::Result<Vec<BotChat>> {
        let mut dialogs_list = Vec::<BotChat>::new();
        let mut dialogs_iter = self.client.iter_dialogs();
        dialogs_iter.total().await?;
        while let Some(dialog) = dialogs_iter.next().await? {
            dialogs_list.push(BotChat {
                chat_id: Option::from(dialog.chat.id()),
                title: Option::from(dialog.chat.name().to_string()),
                tg_chat: None,
            })
        }
        Ok(dialogs_list)
    }

    // TODO Add contact on message send
    // TODO Create chat after adding contact

    pub async fn new(cfg: BotAuth) -> Self {
        println!("Connecting to Telegram...");
        let auth = match cfg {
            BotAuth::TelegramAuth(data) => data,
            _ => {
                println!("No auth data provided!");
                TelegramAuth::default()
            }
        };
        let api_id = auth.app_id;
        let session = Session::load_file_or_create(SESSION_FILE).unwrap();
        let client = Client::connect(Config {
            session,
            api_id,
            api_hash: auth.app_hash.clone(),
            params: Default::default(),
        }).await.unwrap();
        Telegram {
            client,
            handlers: Default::default(),
        }
    }
}

#[async_trait]
impl DocaBot for Telegram {
    fn add_handler(&mut self, user: UserData, handler: BotHandler) {
        if user.messenger_id.is_none() {
            return;
        }
        self.handlers.insert(user.messenger_id.unwrap(), handler);
    }

    async fn handle_message(&self, user: String, message: String) -> utils::Result<()> {
        let handlers = self.handlers.get(&user);
        if handlers.is_none() {
            return Ok(());
        }
        let handler = handlers.unwrap().get(&message);
        if handler.is_none() {
            return Ok(());
        }
        let request = handler.unwrap();
        reqwest::Client::new()
            .post(request.api_url.as_str())
            .body(serde_json::to_string(request).unwrap())
            .send()
            .await.unwrap();
        Ok(())
    }

    async fn sign_in(&self, data: AuthData) -> utils::Result<()> {
        if self.client.is_authorized().await? { return Ok(()); }
        println!("Signing in...");
        let token = self.client.request_login_code(&data.username).await?;
        let signed_in = self.client.sign_in(&token, &data.verify_code).await;
        match signed_in {
            Err(SignInError::PasswordRequired(password_token)) => {
                self.client
                    .check_password(password_token, data.password.trim())
                    .await?;
            }
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
        println!("Signed in!");
        match self.client.session().save_to_file(SESSION_FILE) {
            Ok(_) => {}
            Err(e) => {
                println!(
                    "NOTE: failed to save the session, will sign out when done: {}",
                    e
                );
            }
        }
        Ok(())
    }
    async fn sign_out(&self) {
        drop(self.client.sign_out_disconnect().await);
    }
    async fn send_message(&self, data: SendMessageRequest) -> utils::Result<()> {
        let message = InputMessage::from(data.message.as_str());
        let chat_id = data.user.messenger_id.unwrap().parse::<i64>().unwrap();
        self.client.send_message(PackedChat {
            id: chat_id,
            ty: PackedType::User,
            access_hash: data.access_hash,
        }, message).await?;
        Ok(())
    }

    async fn add_contact(&self, data: AddContactRequest) -> utils::Result<i64> {
        let mut test_import = Vec::new();
        test_import.push(InputContact::InputPhoneContact(
            InputPhoneContact {
                last_name: data.last_name,
                first_name: data.first_name,
                client_id: 0,
                phone: data.phone,
            }
        ));
        match self.client.invoke(&grammers_tl_types::functions::contacts::ImportContacts {
            contacts: test_import
        }).await {
            Ok(grammers_tl_types::enums::contacts::ImportedContacts::Contacts(data)) => {
                let imported_contact = data.imported.first().unwrap();
                match imported_contact {
                    grammers_tl_types::enums::ImportedContact::Contact(data) => Ok(data.user_id),
                    _ => Ok(0)
                }
            }
            Err(e) => {
                println!("{:?}", e);
                Ok(0)
            }
        }
    }

    async fn get_updates(&self) -> Result<Option<Update>, InvocationError> {
        Ok(self.client.next_update().await?)
    }

    async fn message_handler(&self, tx: tokio::sync::mpsc::Sender<ChannelData>) -> utils::Result<()> {
        loop {
            let update = {
                let exit = pin!(async { tokio::signal::ctrl_c().await });
                let upd = pin!(async { self.get_updates().await });
                match select(exit, upd).await {
                    Either::Left(_) => None,
                    Either::Right((u, _)) => Some(u),
                }
            };
            let update = match update {
                None | Some(Ok(None)) => continue,
                Some(u) => u.unwrap(),
            }.unwrap();
            match update {
                Update::MessageEdited(message) => {
                    let message_ref = message.clone();
                    if message.msg.reactions.is_none() {
                        continue;
                    }
                    let grammers_tl_types::enums::MessageReactions::Reactions(reactions) = message.msg.reactions.unwrap();
                    if reactions.results.is_empty() {
                        continue;
                    }
                    let user_reaction = reactions.results.first().unwrap();
                    let grammers_tl_types::enums::ReactionCount::Count(reaction_count) = user_reaction;
                    match &reaction_count.reaction {
                        grammers_tl_types::enums::Reaction::Emoji(emoji) => {
                            let chat = Option::from(PackedChat::from(message_ref.chat()));
                            let data = ReceivedMessage {
                                bot: String::from("telegram"),
                                user: chat.unwrap().id.to_string(),
                                message: emoji.emoticon.clone(),
                            };
                            tx.send(ChannelData::Message(data)).await?;
                            // self.handle_message(data.user, data.message).await.unwrap();
                        }
                        _ => {}
                    }
                }
                Update::NewMessage(message) if !message.outgoing() => {
                    match message.chat().pack().ty {
                        PackedType::User => {
                            let chat = Option::from(PackedChat::from(message.chat()));
                            let data = ReceivedMessage {
                                bot: String::from("telegram"),
                                user: chat.unwrap().id.to_string(),
                                message: String::from(message.text()),
                            };
                            tx.send(ChannelData::Message(data)).await?;
                            // self.handle_message(data.user, data.message).await.unwrap();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    fn get_bot_name(self) -> String {
        String::from("telegram")
    }

    fn clone_boxed(&self) -> Box<dyn DocaBot + 'static> {
        Box::new(self.clone())
    }
}