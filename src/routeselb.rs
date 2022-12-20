use actix_web::web::Json;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct TransReq {
    content: String,
}

impl TransReq {
    pub fn new_string(content: String) -> TransReq {
        TransReq { content }
    }

    pub fn new(content: &str) -> TransReq {
        TransReq {
            content: content.to_string(),
        }
    }

    pub fn example() -> String {
        serde_json::to_string(&TransReq::new("content to translate")).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransResponse {
    content: String,
}

impl TransResponse {
    pub fn new_string(content: String) -> TransResponse {
        TransResponse { content }
    }

    pub fn new(content: &str) -> TransResponse {
        TransResponse {
            content: content.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransErr {
    content: String,
}

impl TransErr {
    pub fn new_string(content: String) -> TransErr {
        TransErr { content }
    }

    pub fn new(content: &str) -> TransErr {
        TransErr {
            content: content.to_string(),
        }
    }
}

#[post("/translate")]
pub async fn translate(req_body: String) -> impl Responder {
    let extracted: TransReq = serde_json::from_str(&req_body).unwrap();

    let result = translate_content(&extracted.content);
    let extracted_result = result.map(|r| serde_json::to_string(&r).unwrap());
    match extracted_result {
        Ok(t) => HttpResponse::Ok().body(t),
        Err(e) => HttpResponse::BadRequest().body(e.content),
    }
}

#[get("/example")]
pub async fn example() -> impl Responder {
    HttpResponse::Ok().body(TransReq::example())
}

#[get("/ready")]
pub async fn ready() -> impl Responder {
    HttpResponse::Ok().body("ready to response")
}

pub fn translate_content(content: &String) -> Result<TransResponse, TransErr> {
    match content.as_str() {
        "" => Err(TransErr::new("Blank translation content")),
        _ => Ok(TransResponse::new_string(content.clone())),
    }
}
