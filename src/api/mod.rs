use std::env::args;
use actix_web::{HttpResponse, post, Responder, web};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::bot::{BotChat, DocaBot};
use crate::structs::api::{AddContactRequest, AppData, BotRequestType, ChannelData, SendMessageRequest, UserHandler};
use crate::bot::telegram::TelegramBot;
use crate::structs::api::BotRequestType::RequestMessage;


#[post("send_message")]
async fn send_message(request: actix_web::web::Json<SendMessageRequest>, app_data: web::Data<AppData>) -> impl Responder {
    let request_ref = request.0.clone();
    match request.0.handlers.clone() {
        Some(data) => {
            let handler = UserHandler{
                user: request.user.clone(),
                handler: data
            };
            app_data.tx.send(ChannelData::Handler(handler)).await.unwrap()
        },
        _ => {}
    };
    let reply = match app_data.bot.send_message(request_ref).await {
        Ok(_) => json!({ "status": 200 }),
        Err(e) => json!({ "status": 500, "details": e.to_string() })
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(reply.to_string())
}

#[post("add_contact")]
async fn add_contact(request: actix_web::web::Json<AddContactRequest>, app_data: web::Data<AppData>) -> impl Responder {
    let reply = match app_data.bot.add_contact(request.0).await {
        Ok(_) => json!({ "status": 200 }),
        Err(e) => json!({ "status": 500, "details": e.to_string() })
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(reply.to_string())
}