use std::env::args;
use actix_web::{HttpResponse, post, Responder, web};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::structs::api::{AddContactRequest, AppData, BotRequestType, ChannelData, SendMessageRequest, UserHandler};
use crate::bot::{BotChat, DocaBot};
use crate::bot::telegram::Telegram;
use crate::structs::api::BotRequestType::RequestMessage;


#[post("send_message")]
async fn send_message(request: web::Json<SendMessageRequest>, app_data: web::Data<AppData>) -> impl Responder {
    let request_ref = request.0.clone();
    let mut bot_statuses = json!({});
    for (bot_name, bot) in app_data.bots.iter() {
        if request_ref.messenger != "*" && bot_name != &request_ref.messenger {
            continue;
        }
        app_data.tx.send(ChannelData::SendMessage(request.0.clone())).await.unwrap();
        bot_statuses[bot_name] = json!({ "status": 200 });
    }
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(bot_statuses.to_string())
}

#[post("add_contact")]
async fn add_contact(request: web::Json<AddContactRequest>, app_data: web::Data<AppData>) -> impl Responder {
    let request_ref = request.0.clone();
    let mut bot_statuses = json!({});
    for (bot_name, bot) in app_data.bots.iter() {
        if request_ref.messenger != "*" && bot_name != &request.messenger {
            continue;
        }
        bot_statuses[bot_name] = match bot.add_contact(request.0.clone()).await {
            Ok(user) => json!({ "id": user.to_string() }),
            Err(e) => json!({ "status": 500, "details": e.to_string() })
        };
    }
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(bot_statuses.to_string())
}