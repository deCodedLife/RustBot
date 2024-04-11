use serde::{Deserialize, Serialize};
use crate::utils::JsonConfigs;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) verify_code: String
}

impl JsonConfigs for AuthData{}