mod time_stamp;

use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64url::encode;

pub struct CustomJwt {
    secret: String,
    header: String,
    payload: String,
}

impl CustomJwt {
    pub fn new(user_id: &str, user_secret: &str) -> CustomJwt {
        let header = "{\"alg\":\"HS256\",\"sign_type\":\"SIGN\"}".to_string();
        let payload = CustomJwt::jwt_payload(user_id);
        CustomJwt {
            secret: user_secret.to_string(),
            header,
            payload,
        }
    }

    pub fn create_jwt(&self) -> St