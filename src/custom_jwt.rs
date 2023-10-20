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

    pub fn create_jwt(&self) -> String {
        let encoded_header = CustomJwt::encode_base64_url(self.header.as_bytes());
        let encoded_payload = CustomJwt::encode_base64_url(self.payload.as_bytes());
        let to_sign = format!("{}.{}", encode