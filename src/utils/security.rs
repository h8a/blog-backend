use chrono::{Duration, Utc};
use jsonwebtoken::{
    // decode_header,
    decode,
    encode,
    Algorithm,
    DecodingKey,
    EncodingKey,
    Header,
    Validation
};
use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, SaltString, PasswordVerifier
    },
    Pbkdf2
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: Option<String>,         // Optional. Audience
    pub exp: usize,                  // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: Option<usize>,          // Optional. Issued at (as UTC timestamp)
    pub iss: Option<String>,         // Optional. Issuer
    pub nbf: Option<usize>,          // Optional. Not Before (as UTC timestamp)
    pub sub: Option<String>,         // Optional. Subject (whom token refers to)
    pub id: String,
}

pub fn jwt_encode(user_id: &i32) -> String {

    let exp_seconds: i64 = dotenv::var("JWT_EXP").unwrap().parse::<i64>().unwrap();

    let expiration = Utc::now() + Duration::seconds(exp_seconds);

    let claims = Claims {
        exp: expiration.timestamp() as usize,
        aud: None,
        iat: None,
        iss: None,
        nbf: None,
        sub: None,
        id: user_id.to_string(),
    };

    encode(
        &Header::new(Algorithm::HS512), 
        &claims, 
        &EncodingKey::from_secret(dotenv::var("JWT_SECRET")
            .unwrap()
            .as_ref())
        )
        .unwrap()
}

pub fn jwt_decode(token: &str) -> Result<i32, bool> {
    // println!("{:?}", decode_header(&token));
    match decode::<Claims>(
            &token, 
            &DecodingKey::from_secret(dotenv::var("JWT_SECRET").unwrap().as_ref()
        ), 
        &Validation::new(Algorithm::HS512)
    ) {
        Ok(user) => Ok(user.claims.id.parse::<i32>().unwrap()),
        Err(_) => Err(false)
    }
}

pub fn encode_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Pbkdf2.hash_password(password, &salt).unwrap().to_string()
}

pub fn decode_password(password_hash: &str, password: &[u8]) -> bool {
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    Pbkdf2.verify_password(password, &parsed_hash).is_ok()
}