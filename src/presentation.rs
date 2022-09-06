use std::{future::Future, pin::Pin};

use actix_web::{get, http::header, web::ServiceConfig, Error, FromRequest, Responder};
use lazy_static::lazy_static;
use regex::Regex;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[get("/")]
async fn index(bearer_token: BearerToken) -> impl Responder {
    println!("{:?}", bearer_token);
    format!("HELLO WORLD!")
}

lazy_static! {
    static ref BEARER_REGEXP: Regex = Regex::new(r"^Bearer\s(.*)$").unwrap();
}

#[derive(Debug)]
pub struct BearerToken(Option<String>);

impl FromRequest for BearerToken {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let token = req
                .headers()
                .get(header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|authorization| {
                    BEARER_REGEXP
                        .captures(authorization)
                        .and_then(|captures| captures.get(1))
                })
                .map(|v| v.as_str().to_owned());

            Ok(BearerToken(token))
        })
    }
}
