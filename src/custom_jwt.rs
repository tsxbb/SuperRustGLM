mod time_stamp;

use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64url::encode;

pub struct CustomJwt {
    secret: String,
    header: String,
    payload: String,
}

impl Custo