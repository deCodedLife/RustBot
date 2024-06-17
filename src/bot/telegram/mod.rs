use std::io;
use std::io::{BufRead, Write};
use std::pin::pin;
use async_trait::async_trait;
use futures_util::future::{Either, select};
use grammers_client::{Client, Config, InputMessage, SignInError, Update};
use grammers_mtsender::InvocationError;
use grammers_session::{PackedChat, PackedType, Session};
use grammers_tl_types::enums::{InputContact};
use grammers_tl_types::types::{InputPhoneContact};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::bot::{BotAuth, BotChat, BotContact, DocaBot};
use crate::structs::auth::AuthData;
use crate::{SESSION_FOLDER, utils};
use crate::structs::api::{AddContactRequest, SendMessageRequest, BotHandler, UserHandlers, ChannelData, ReceivedMessage, UserData, BotContext};
use crate::utils::JsonConfigs;

#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TelegramAuth {
    pub app_id: i32,
    pub app_hash: String,
}

impl JsonConfigs for TelegramAuth {}

#[derive(Clone)]
pub struct Telegram {
    pub client: Client,
    pub bot_id: i64,
    pub handlers: UserHandlers,
}

fn prompt(message: &str) -> utils::Result<String> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut line = String::new();
    stdin.read_line(&mut line)?;
    Ok(line)
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

    pub async fn new(bot_name: String, cfg: BotAuth) -> Self {
        println!("Connecting to Telegram...");
        let auth = match cfg {
            BotAuth::TelegramAuth(data) => data,
            _ => {
                println!("No auth data provided!");
                TelegramAuth::default()
            }
        };
        let api_id = auth.app_id;
        let session_file = format!("configs/{}/{}.session", SESSION_FOLDER, bot_name);
        let session = Session::load_file_or_create(session_file).unwrap();
        let client = Client::connect(Config {
            session,
            api_id,
            api_hash: auth.app_hash.clone(),
            params: Default::default(),
        }).await.unwrap();
        let bot_id = client.get_me().await.unwrap().id();
        Telegram {
            client,
            bot_id,
            handlers: Default::default(),
        }
    }
}

#[async_trait]
impl DocaBot for Telegram {
    fn get_bot_name(self) -> String {
        String::from("telegram")
    }

    fn add_handler(&mut self, user: UserData, handler: BotHandler) {
        if user.messenger_id.is_none() {
            return;
        }
        self.handlers.insert(user.messenger_id.unwrap(), handler);
    }

    async fn sign_in(&self, bot_name: String, data: AuthData) -> utils::Result<()> {
        if self.client.is_authorized().await? { return Ok(()); }
        let AuthData::Telegram(auth_data) = data else { return Ok(()) };
        println!("Signing in...");
        let token = self.client.request_login_code(&auth_data.username).await?;

        let prompt_text = format!( "{} Code: ", bot_name );
        let code = prompt( &prompt_text ).unwrap();
        let signed_in = self.client.sign_in(&token, &code).await;
        match signed_in {
            Err(SignInError::PasswordRequired(password_token)) => {
                self.client
                    .check_password(password_token, auth_data.password.trim())
                    .await?;
            }
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        };
        println!("Signed in!");
        let session_file = format!("configs/{}/{}.session", SESSION_FOLDER, bot_name);
        match self.client.session().save_to_file(session_file) {
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
        if data.user.messenger_id.is_none() {
            return Ok(());
        }
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
                if data.imported.first().is_none() {
                    return Ok(0)
                }
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

    async fn message_handler(&self, ctx: BotContext, tx: tokio::sync::mpsc::Sender<ChannelData>) -> utils::Result<()> {
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
                Some(u) => match u {
                    Ok(update) => update,
                    Err(_) => continue
                },
            }.unwrap();
            match update {
                Update::NewMessage(message) if !message.outgoing() => {
                    match message.chat().pack().ty {
                        PackedType::User => {
                            let chat = Option::from(PackedChat::from(message.chat()));
                            let user = chat.unwrap().id.to_string();
                            if user == self.bot_id.to_string() {
                                continue
                            }
                            let data = ReceivedMessage {
                                ctx: ctx.clone(),
                                user: user.clone(),
                                message: String::from(message.text()),
                            };
                            tx.send(ChannelData::ReceiveMessage(data)).await?;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    async fn handle_message(&mut self, user: String, ctx: BotContext, message: String) -> utils::Result<()> {
        if !message.contains("1") {
            return Ok(());
        }
        let request = json!({
            "object": "visits",
            "command": "bot_verify",
            "data": {
                "context": {
                    "bot": true,
                    "user_id": &user,
                }
            }
        });
        reqwest::Client::new()
            .post(ctx.api_url)
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await.unwrap();
        Ok(())
    }

    async fn delete_contacts(&self) {
        let response = self.client.invoke(&grammers_tl_types::functions::contacts::GetContacts{
            hash: 0
        }).await.unwrap();
        let mut delete_users: Vec<grammers_tl_types::enums::InputUser> = Vec::new();
        let grammers_tl_types::enums::contacts::Contacts::Contacts(contacts) = response else { return; };
        for user_data in contacts.users.iter() {
            let grammers_tl_types::enums::User::User(user) = user_data else { continue };
            let user_to_delete = grammers_tl_types::types::InputUser {
                user_id: user.id,
                access_hash: user.access_hash.unwrap(),
            };
            delete_users.push(grammers_tl_types::enums::InputUser::User(user_to_delete));
        }
        self.client.invoke(&grammers_tl_types::functions::contacts::DeleteContacts{
            id: delete_users
        }).await.unwrap();
    }

    fn clone_boxed(&self) -> Box<dyn DocaBot + 'static> {
        Box::new(self.clone())
    }
}