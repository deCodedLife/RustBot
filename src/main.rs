use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use simple_logger::SimpleLogger;
use crate::bot::{BotAuth, DocaBot};
use crate::bot::telegram::{Telegram, TelegramAuth};
use crate::structs::api::{AppData, BotContext};
use crate::structs::auth::{AuthData, AuthList};
use crate::structs::wrapper::ChannelTx;
use crate::utils::JsonConfigs;
use crate::wrapper::wrapper::{BotStorage, Wrapper};

pub mod structs;

#[cfg(test)]
mod tests;
mod bot;
mod utils;
mod api;
mod wrapper;

// const SESSION_FILE: &str = "community_telegram.session";
const SESSION_FOLDER: &str = "sessions";



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

    let mut bot_list: BotStorage = HashMap::new();
    let app_data  = TelegramAuth::from_file("configs/telegram.json");

    for ( bot_name, auth_data ) in get_configs("configs/auth_data.json").iter() {
        let mut bot = Telegram::new(
            bot_name.clone(),
            BotAuth::TelegramAuth(app_data.clone()),
            BotContext{
                bot_name: bot_name.clone(),
                api_url: auth_data.api_url.clone()
        }).await;
        bot.sign_in(bot_name.clone(), AuthData::Telegram(auth_data.clone())).await.unwrap();
        bot.dialogs = bot.get_dialogs().await.unwrap();
        bot_list.insert(bot_name.clone(), Box::new(bot.clone()));
    };

    let (bot_tx, bot_rx) = tokio::sync::mpsc::channel::<ChannelTx>(4096);
    let bot_list: Arc<BotStorage> = Arc::new(bot_list);

    for (_, bot_instance) in bot_list.iter() {
        let bot_clone: Arc<Box<dyn DocaBot>> = Arc::new(bot_instance.clone());
        let tx_clone = bot_tx.clone();
        actix_rt::spawn(async move {
            bot_clone.message_handler(tx_clone).await;
            0
        });
    }

    let wrapper = Wrapper::new(bot_list, bot_rx);
    Wrapper::exec(Arc::<Wrapper>::new(wrapper));

    HttpServer::new(move || {
        let app_data = AppData {
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
