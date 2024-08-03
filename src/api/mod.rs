use actix_web::{HttpResponse, post, Responder, web};
use actix_web::http::header::ContentType;
use serde_json::{json, Value};
use crate::structs::api::{AddContactRequest, AppData, SendMessageRequest};
use crate::structs::wrapper::{ChannelData, ChannelTx};


#[post("send_message")]
async fn send_message(request: web::Json<SendMessageRequest>, app_data: web::Data<AppData>) -> impl Responder {
    let request_ref = request.0.clone();
    let tx_result = app_data.tx.send(ChannelTx{
        bot_name: request_ref.messenger,
        data: ChannelData::SendMessage(request.0.clone())
    }).await;
    let result: Value = match tx_result {
        Ok(_) => json!({ "status": 200 }),
        Err(e) => {
            let error = e.to_string();
            json!({ "status": error })
        }
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(result.to_string())
}

#[post("add_contact")]
async fn add_contact(request: web::Json<AddContactRequest>, app_data: web::Data<AppData>) -> impl Responder {
    let request_ref = request.0.clone();
    let tx_result = app_data.tx.send(ChannelTx{
        bot_name: request_ref.messenger,
        data: ChannelData::AddContact(request.0.clone())
    }).await;
    let result: Value = match tx_result {
        Ok(_) => json!({ "status": 200 }),
        Err(e) => {
            let error = e.to_string();
            json!({ "status": error })
        }
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(result.to_string())
}