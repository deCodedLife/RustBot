use std::pin::pin;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use actix_web::{App, HttpServer, web};
use futures_util::future::{Either, select};
use futures_util::TryFutureExt;
use grammers_client::{InputMessage, Update};
use grammers_session::{PackedChat, PackedType};
use grammers_tl_types::enums::SecureValue::Value;
// use std::io;
use simple_logger::SimpleLogger;
use tokio::{runtime, task};
// mod bot;

use crate::bot::{BotChat, DocaBot};
use crate::bot::telegram::TelegramBot;
use crate::structs::*;
use crate::structs::api::BotRequest;
use crate::utils::JsonConfigs;

pub mod structs;

#[cfg(test)]
mod tests;
mod bot;
mod utils;
mod api;

const SESSION_FILE: &str = "dialogs.session";

// fn prompt(message: &str) -> utils::Result<String> {
//     let stdout = io::stdout();
//     let mut stdout = stdout.lock();
//     stdout.write_all(message.as_bytes())?;
//     stdout.flush()?;
//
//     let stdin = io::stdin();
//     let mut stdin = stdin.lock();
//
//     let mut line = String::new();
//     stdin.read_line(&mut line)?;
//     Ok(line)
// }
type Result = std::result::Result<(), Box<dyn std::error::Error>>;
async fn echo_messages<T: bot::DocaBot>(bot: T) -> Result {
    loop {
        let update = {
            let exit = pin!(async { tokio::signal::ctrl_c().await });
            let upd = pin!(async { bot.get_updates().await });

            match select(exit, upd).await {
                Either::Left(_) => None,
                Either::Right((u, _)) => Some(u),
            }
        };

        let update = match update {
            None | Some(Ok(None)) => break,
            Some(u) => u?.unwrap(),
        };

        match update {
            Update::NewMessage(message) if !message.outgoing() => {
                match message.chat().pack().ty {
                    PackedType::User => {
                        let chat = Option::from( PackedChat::from( message.chat() ) );
                        if message.text() != "Пользователь сейчас занят разработкой бота)" {
                            bot.send_message( &BotRequest {
                                messenger: String::from("Telegram"),
                                user: chat.unwrap().id.to_string(),
                                message: String::from("Пользователь сейчас занят разработкой бота)"),
                                buttons: None,
                                handlers: None
                            } ).await?;
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    Ok(())
}


async fn async_main() -> std::io::Result<()> {

    let bot_config = bot::BotAuth::from_file("configs/app_configs.json");
    let user_data = auth::AuthData::from_file("configs/user_config.json");

    let mut bot = bot::telegram::TelegramBot::new(bot_config).await;
    bot.sign_in(user_data).await.unwrap();
    actix_rt::spawn( echo_messages(bot.clone()).into_future() );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(bot.clone()))
            .service(api::send_message)
    })
        .bind(("127.0.0.1", 8081))?
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
