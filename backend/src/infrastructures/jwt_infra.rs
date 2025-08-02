use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use shared::models::claims_model::Claims;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn issue_token(secret: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600; // 1 ชั่วโมง

    let claims = Claims {
        sub: "user123".to_string(),
        roles: vec!["admin".to_string()],
        permissions: vec!["edit_user".to_string()],
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

pub fn verify_token(token: &str, secret: &str) -> Option<Claims> {
    let result = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    );

    match result {
        Ok(data) => Some(data.claims),
        Err(err) => {
            eprintln!("JWT Decode error: {:?}", err);
            None
        }
    }
}
