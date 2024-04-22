use std::collections::HashMap;
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
use crate::structs::api::{AppData, ChannelData};
use crate::utils::JsonConfigs;

pub mod structs;

#[cfg(test)]
mod tests;
mod bot;
mod utils;
mod api;

const SESSION_FILE: &str = "dialogs.session";


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
            ChannelData::Message(data) => {
                let bot_instance = bot.get(&data.bot);
                if bot_instance.is_none() {
                    continue;
                }
                bot_instance.unwrap().handle_message(data.user, data.message).await?;
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

async fn async_main() -> std::io::Result<()> {
    let bot_config = TelegramAuth::from_file("configs/telegram.json");
    // let bot_config2 = WhatsappAuth::from_file("configs/whatsapp.json");

    let user_data = auth::AuthData::from_file("configs/user_config.json");
    let telegram = Telegram::new(BotAuth::TelegramAuth(bot_config)).await;
    telegram.sign_in(user_data).await.unwrap();

    // let whatsapp = WhatsApp::new(BotAuth::WhatsappAuth(bot_config2)).await;

    let mut bot_list: HashMap<String, Box<dyn DocaBot>> = HashMap::new();
    bot_list.insert(String::from("telegram"), Box::new(telegram.clone()));

    let (bot_tx, mut bot_rx) = tokio::sync::mpsc::channel::<ChannelData>(1024);
    let test = bot_tx.clone();
    let bot = telegram.clone();
    actix_rt::spawn(handle_messages(bot_list.clone(), bot_rx).into_future());

    for (_, bot_instance) in bot_list.clone() {
        let bot_clone = bot_instance.clone();
        let tx_clone = bot_tx.clone();
        actix_rt::spawn(async move{
            bot_clone.message_handler(tx_clone)
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
