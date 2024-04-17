use std::future::IntoFuture;
use std::ops::Deref;
use std::pin::pin;
use actix_web::{App, HttpServer, web};
use futures_util::future::{Either, select};
pub use grammers_client::{Update};
use grammers_session::{PackedChat, PackedType};
use simple_logger::SimpleLogger;
use tokio::sync::mpsc::Receiver;


use crate::bot::{BotAuth, DocaBot};
use crate::bot::telegram_user::{TelegramUser, TelegramUserAuth};
use crate::structs::*;
use crate::structs::api::{AppData, ChannelData, ReceivedMessage};
use crate::utils::JsonConfigs;

pub mod structs;

#[cfg(test)]
mod tests;
mod bot;
mod utils;
mod api;

const SESSION_FILE: &str = "dialogs.session";

async fn get_updates(bot: TelegramUser, tx: tokio::sync::mpsc::Sender<ChannelData>) -> utils::Result<()> {
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
            None | Some(Ok(None)) => continue,
            Some(u) => u?,
        }.unwrap();
        match update {
            Update::MessageEdited(message) => {
                let message_ref = message.clone();
                if message.msg.reactions.is_none() {
                    continue;
                }
                let grammers_tl_types::enums::MessageReactions::Reactions(reactions) = message.msg.reactions.unwrap();
                let user_reaction = reactions.results.first().unwrap();
                let grammers_tl_types::enums::ReactionCount::Count(reaction_count) = user_reaction;
                match &reaction_count.reaction {
                    grammers_tl_types::enums::Reaction::Emoji(emoji) => {
                        let chat = Option::from(PackedChat::from(message_ref.chat()));
                        let message = ReceivedMessage {
                            user: chat.unwrap().id.to_string(),
                            message: emoji.emoticon.clone()
                        };
                        tx.send(ChannelData::Message(message)).await.unwrap();
                    }
                    _ => {}
                }
            }
            Update::NewMessage(message) if !message.outgoing() => {
                match message.chat().pack().ty {
                    PackedType::User => {
                        let chat = Option::from(PackedChat::from( message.chat() ) );
                        let message = ReceivedMessage {
                            user: chat.unwrap().id.to_string(),
                            message: String::from(message.text())
                        };
                        tx.send(ChannelData::Message(message)).await.unwrap();
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

async fn handle_messages(mut bot: TelegramUser, mut bot_rx: Receiver<ChannelData>, ) -> utils::Result<()> {
    loop {
        let data = bot_rx.recv().await;
        if data.is_none() {
            continue
        }
        match data.unwrap() {
            ChannelData::Handler(data) => bot.add_handler(data.user, data.handler),
            ChannelData::Message(data) => bot.handle_message(data.user, data.message).await.unwrap(),
            _ => {}
        };
    }
}


async fn async_main() -> std::io::Result<()> {
    let bot_config = TelegramUserAuth::from_file("configs/app_configs.json");
    let user_data = auth::AuthData::from_file("configs/user_config.json");
    let bot = bot::telegram_user::TelegramUser::new(BotAuth::TelegramUser(bot_config)).await;
    bot.sign_in(user_data).await.unwrap();

    let (bot_tx, mut bot_rx) = tokio::sync::mpsc::channel::<ChannelData>(1024);
    actix_rt::spawn(get_updates(bot.clone(), bot_tx.clone()).into_future());
    actix_rt::spawn(handle_messages(bot.clone(), bot_rx).into_future().into_future());

    HttpServer::new(move || {
        let app_data = AppData {
            bot: bot.clone(),
            tx: bot_tx.clone()
        };
        App::new()
            .app_data(web::Data::new(app_data))
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
