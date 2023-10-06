use chrono::Utc;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::serde::{Deserialize, Serialize};
use rocket::Responder;
//use shared::response_models::{Response, ResponseBody, NetworkResponse};
use rocket::request::{Outcome, Request, FromRequest};
use rocket::http::Status;
//use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: i32,
    exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

pub struct User {
    pub id: i32,
    pub user_name: String,
    pub password: String,
}

pub fn create_jwt(id: i32) -> Result<String, Error> {
    let secret = "TODO env::var(\"JWT_SECRET\").expect(\"JWT_SECRET must be set.\")"; // TODO
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = "TODO env::var(\"JWT_SECRET\").expect(\"JWT_SECRET must be set.\")"; // TODO
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        match req.headers().get_one("authorization") {
            None => {
                let response = Response {
                    body: ResponseBody::Message(
                        String::from("Error validating JWT token - No token provided")
                    )
                };

                Outcome::Failure((
                    Status::Unauthorized,
                    NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                ))
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT {claims}),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response {
                            body: ResponseBody::Message(
                                format!("Error validating JWT token - Expired Token")
                            )
                        };

                        Outcome::Failure((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        ))
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response {
                            body: ResponseBody::Message(
                                format!("Error validating JWT token - Invalid Token")
                            )
                        };

                        Outcome::Failure((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        ))
                    },
                _ => {
                        let response = Response {
                            body: ResponseBody::Message(
                                format!("Error validating JWT token - {}", err)
                            )
                        };

                        Outcome::Failure((
                            Status::Unauthorized,
                            NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap())
                        ))
                    }
                }
            },
        }
    }
}

#[get("/jwt/<post_id>")]
pub fn publish_post_handler(post_id: i32, key: Result<JWT, NetworkResponse>) -> Result<String, NetworkResponse> {
    // 👇 New!
    let key = key?;
    // let post = publish::publish_post(post_id, key)?;
    let response = Response { body: ResponseBody::Message("post".to_string()) };
    Ok(serde_json::to_string(&response).unwrap())
}
