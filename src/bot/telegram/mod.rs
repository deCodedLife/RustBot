use std::ops::Deref;
use grammers_client::{Client, Config, InputMessage, SignInError, Update};
use grammers_client::client::dialogs;
use grammers_client::types::Chat::Channel;
use grammers_mtsender::InvocationError;
use grammers_session::{PackedChat, PackedType, Session};
use grammers_tl_types::enums::{Chat, InputPeer};
use grammers_tl_types::Serializable;
use grammers_tl_types::serialize::Buffer;
use serde::Serialize;
use serde_json::Value;
use crate::bot::{BotAuth, BotChat, BotContact, DocaBot};
use crate::structs::auth::AuthData;
use crate::{SESSION_FILE, utils};
use crate::structs::api::BotRequest;

#[derive(Clone)]
pub struct TelegramBot {
    pub client: Client
}

impl TelegramBot {
    async fn import_contact() {
        // let mut test_import = Vec::new();
        // test_import.push( InputContact::InputPhoneContact(
        //     InputPhoneContact{
        //         last_name: "Test".to_string(),
        //         first_name: "Test".to_string(),
        //         client_id: 0,
        //         phone: "+79786588286".to_string()
        //     }
        // ) );
        // client.invoke(&grammers_tl_types::functions::contacts::ImportContacts {
        //     contacts: test_import
        // }).await?;
    }
}

impl DocaBot for TelegramBot {
    async fn new(cfg: BotAuth) -> Self {
        println!("Connecting to Telegram...");
        let api_id = cfg.app_id;
        let session = Session::load_file_or_create(SESSION_FILE).unwrap();
        let client = Client::connect(Config {
            session,
            api_id,
            api_hash: cfg.app_hash.clone(),
            params: Default::default(),
        }).await.unwrap();
        TelegramBot { client }
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

    async fn get_dialogs(&self) -> utils::Result<Vec<BotChat>> {
        let mut dialogs_list = Vec::<BotChat>::new();
        let mut dialogs_iter = self.client.iter_dialogs();
        dialogs_iter.total().await?;

        while let Some(dialog) = dialogs_iter.next().await? {
            dialogs_list.push(BotChat{
                chat_id: Option::from( dialog.chat.id() ),
                title: Option::from( dialog.chat.name().to_string() ),
                tg_chat: None
            })
        }

        Ok(dialogs_list)
    }

    async fn get_contacts(&self) -> utils::Result<Vec<BotContact>> {
        let contacts = Vec::<BotContact>::new();
        let _ = self.client.invoke( &grammers_tl_types::functions::contacts::GetContacts{ hash: 0 } ).await?;
        Ok(contacts)
    }

    async fn send_message(&self, data: &BotRequest) -> utils::Result<()> {
        let message = InputMessage::from(data.message.as_str());
        if data.buttons.is_some() {
            // TODO
        }
        self.client.send_message(PackedChat{
            id: data.user.parse::<i64>().unwrap(),
            ty: PackedType::User,
            access_hash: None
        }, message ).await?;
        Ok(())
    }

    async fn get_updates(&self) -> Result<Option<Update>, InvocationError> {
        Ok(self.client.next_update().await?)
    }
}