use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::future::IntoFuture;
use std::hash::Hash;
use std::ops::Deref;
use actix_web::{App, HttpServer, web};
use simple_logger::SimpleLogger;
use tokio::sync::mpsc::{Receiver};


use crate::bot::{BotAuth, DocaBot};
use crate::bot::telegram::{Telegram, TelegramAuth};
use crate::bot::whatsapp::{WhatsApp, WhatsappAuth};
use crate::structs::*;
use crate::structs::api::{AppData, BotContext, ChannelData};
use crate::structs::auth::{AuthData, AuthList};
use crate::utils::JsonConfigs;

pub mod structs;

#[cfg(test)]
mod tests;
mod bot;
mod utils;
mod api;

const SESSION_FILE: &str = "community_telegram.session";
const SESSION_FOLDER: &str = "sessions";





async fn handle_messages(mut bot: HashMap<String, Box<dyn DocaBot>>, mut bot_rx: Receiver<ChannelData>) -> utils::Result<()> {
    loop {
        let data = bot_rx.recv().await;
        if data.is_none() {
            continue
        }
        match data.unwrap() {
            ChannelData::Handler(data) => {
                bot.entry(data.bot).and_modify( |te| {
                    te.add_handler(data.user, data.handler)
                } );
            },
            ChannelData::SendMessage(data) => {
                let mut bot_instance = bot.get_mut(&data.messenger);
                if bot_instance.is_none() {
                    continue;
                }
                bot_instance.unwrap().send_message(data).await?;
            },
            ChannelData::ReceiveMessage(data) => {
                let mut bot_instance = bot.get_mut(&data.ctx.bot_name);
                if bot_instance.is_none() {
                    continue;
                }
                bot_instance.unwrap().handle_message(
                    data.message.user.clone(),
                    data.ctx.clone(),
                    data.message.text.clone()).await?;
            }
            _ => {}
        };
    }
}

// async fn message_handlers(mut bot: HashMap<String, Box<dyn DocaBot>>, mut bot_tx: Sender<ChannelData>) -> utils::Result<()> {
//     for (bot_name, bot) in bot.iter() {
//         actix_rt::spawn(bot.message_handler(bot_tx.clone()).into_future());
//     }
//     Ok(())
// }

fn get_configs(file_name: &str) -> AuthList {
    if fs::metadata(file_name).is_err() {
        println!("[!] {} not found", file_name);
        return AuthList::default();
    }
    let file_contents: String = fs::read_to_string(file_name).unwrap();
    serde_json::from_str::<AuthList>(&file_contents.clone()).unwrap_or_else(|e| {
        println!("[!] Config file is corrupted: {:?}", e);
        AuthList::default()
    })
}

async fn async_main() -> std::io::Result<()> {

    if fs::metadata(format!("configs/{}", SESSION_FOLDER)).is_err() {
        fs::create_dir_all(format!("configs/{}", SESSION_FOLDER)).unwrap();
    }

    let mut bot_list: HashMap<String, Box<dyn DocaBot>> = HashMap::new();
    let mut bot_ctxs: HashMap<String, Box<BotContext>> = HashMap::new();

    let (bot_tx, mut bot_rx) = tokio::sync::mpsc::channel::<ChannelData>(1024);

    let app_data  = TelegramAuth::from_file("configs/telegram.json");
    // let bot_config2 = WhatsappAuth::from_file("configs/whatsapp.json");

    for ( bot_name, auth_data ) in get_configs("configs/auth_data.json").iter() {
        let mut bot = Telegram::new(bot_name.clone(), BotAuth::TelegramAuth(app_data.clone())).await;
        bot.sign_in(bot_name.clone(), AuthData::Telegram(auth_data.clone())).await.unwrap();
        bot.dialogs = bot.get_dialogs().await.unwrap();
        bot_list.insert(bot_name.clone(), Box::new(bot.clone()));
        bot_ctxs.insert(bot_name.clone(), Box::new(BotContext{
            bot_name: bot_name.clone(),
            api_url: auth_data.api_url.clone()
        }));
    };

    actix_rt::spawn(handle_messages(bot_list.clone(), bot_rx).into_future());

    for (bot_name, bot_instance) in bot_list.clone() {
        let bot_ctx = bot_ctxs.clone().get(&bot_name).unwrap().clone();
        let mut bot_clone = bot_instance.clone();
        let tx_clone = bot_tx.clone();
        actix_rt::spawn(async move{
            bot_clone
                .custom_handler(BotContext{
                    bot_name: bot_ctx.bot_name.clone(),
                    api_url: bot_ctx.api_url.clone()
                }, tx_clone)
                .await
        });
    }

    HttpServer::new(move || {
        let app_data = AppData {
            bots: bot_list.clone(),
            tx: bot_tx.clone()
        };
        App::new()
            .app_data(web::Data::new(app_data))
            .service(api::send_message)
            .service(api::add_contact)
    })
        .bind(("127.0.0.1", 1052))?
        .run()
        .await
}

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    actix_rt::System::with_tokio_rt( || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(8)
            .thread_name( "actix" )
            .build()
            .unwrap()
    } ).block_on(async_main()).unwrap();
}
