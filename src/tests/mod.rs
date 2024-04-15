mod sense_data;

use crate::bot;
use crate::structs::*;
use crate::utils::JsonConfigs;
use crate::tests::sense_data::{APP_HASH, APP_ID, PASSWORD, USERNAME};


#[test]
fn auth_data_write() {
    let mut user_data: auth::AuthData = auth::AuthData::default();
    user_data.username = USERNAME;
    user_data.password = PASSWORD;
    user_data.verify_code = "".to_string();
    user_data.into_file("configs/user_config.json").unwrap();
}

#[test]
fn bot_data_write() {

    let mut bot_data: bot::BotAuth = bot::BotAuth::default();
    bot_data.app_id = APP_ID;
    bot_data.app_hash = APP_HASH;
    bot_data.into_file("configs/app_configs.json").unwrap()


}

#[test]
fn test_api_request() {

    // let request = BotRequest::from_file("configs/api_request.json");
    // println!("{:?}", request);

}