use std::env::args;
use actix_web::{HttpResponse, post, Responder, web};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::bot::{BotChat, DocaBot};
use crate::structs::api::BotRequest;
use crate::bot::telegram::TelegramBot;


#[post("send_message")]
async  fn send_message(request: actix_web::web::Json<BotRequest>, bot: web::Data<TelegramBot>) -> impl Responder {
    match bot.send_message(&request).await {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(json!({ "status": 200 }).to_string()),
        Err(e) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(json!({ "status": 500 }).to_string())
    }
}