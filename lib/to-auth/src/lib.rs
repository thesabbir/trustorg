use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use serde::Deserialize;
use sha2::Sha256;
use std::collections::BTreeMap;
use to_config::read_config;

#[derive(Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
}

pub fn local_auth(user: &User) -> bool {
    let config = read_config();
    if user.email == config.admin_email && user.password == config.admin_password {
        return true;
    }
    return false;
}

pub fn get_token(user: &User) -> String {
    let config = read_config();
    let key: Hmac<Sha256> = Hmac::new_from_slice(&config.secrete.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("email", &user.email);
    return claims.sign_with_key(&key).unwrap();
}

pub fn verify_token(token: &str) -> bool {
    let config = read_config();
    let key: Hmac<Sha256> = Hmac::new_from_slice(&config.secrete.as_bytes()).unwrap();
    let claims: Result<BTreeMap<String, String>, jwt::Error> = token.verify_with_key(&key);
    match claims {
        Err(_) => false,
        Ok(_) => true,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
