use std::env::args;
use actix_web::{HttpResponse, post, Responder};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::structs::api::BotRequest;


#[post("send_message")]
async  fn send_message(request: actix_web::web::Json<BotRequest>) -> impl Responder {
    println!( "{:?}", request );
    let mut  resp = json!({ "status": 200 }).to_string();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(resp)
}