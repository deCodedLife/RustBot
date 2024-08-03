use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{Receiver};
use crate::bot::DocaBot;
use crate::structs::wrapper::{ChannelData, ChannelTx};

pub type BotStorage = HashMap<String, Box<dyn DocaBot>>;
pub type BotReceiver = Arc<Mutex<Receiver<ChannelTx>>>;

pub struct Wrapper {
    messengers: Arc<BotStorage>,
    commands_rc: BotReceiver
}

impl Wrapper {
    pub fn new(msg: Arc<BotStorage>, commands: Receiver<ChannelTx>) -> Wrapper {
        return Wrapper {
            messengers: msg,
            commands_rc: BotReceiver::new(Mutex::<Receiver<ChannelTx>>::new(commands))
        }
    }

    async fn internal(&self) {
        loop {
            let data_option: Option<ChannelTx> = self.commands_rc.lock().unwrap().recv().await;
            if data_option.is_none() {
                continue
            }
            let data_unwrapped: ChannelTx = data_option.unwrap();
            let bot_name: String = data_unwrapped.bot_name;
            let command: ChannelData = data_unwrapped.data;
            let bot_instance: Option<&Box<dyn DocaBot>> = self.messengers.get(&bot_name);
            if bot_instance.is_none() {
                continue;
            }
            let _ = match command {
                ChannelData::ReceiveMessage(msg) => bot_instance.unwrap().handle_message(msg.user, msg.text).await,
                ChannelData::SendMessage(msg) => bot_instance.unwrap().send_message(msg).await,
                ChannelData::AddContact(contact) => bot_instance.unwrap().add_contact(contact).await,
                // ChannelData::Handler(handler) => bot_instance.unwrap().add_handler(handler.user, handler.handler),
            };
        }
    }

    pub fn exec(wrapper: Arc<Wrapper>) {
        actix_rt::spawn(async move {
            wrapper.internal().await;
            0
        });
    }
}