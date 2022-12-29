use actix_web::{get, post, HttpResponse, Responder};
use translation_server_dtos_silen::{NewTransReq, TransReq};

use crate::{jsontransreqhandler::handle_trans_req, newtranslations::new_translation};

#[post("/translate")]
pub async fn translate(req_body: String) -> impl Responder {
    handle_trans_req(req_body)
}

#[get("/exampletranslate")]
pub async fn trans_example() -> impl Responder {
    HttpResponse::Ok().body(TransReq::example())
}

#[get("/ready")]
pub async fn ready() -> impl Responder {
    HttpResponse::Ok().body("ready to response")
}

#[get("/examplenewtrans")]
pub async fn new_trans_example() -> impl Responder {
    HttpResponse::Ok().body(NewTransReq::example())
}

#[post("/newtrans")]
pub async fn new_trans(req_body: String) -> impl Responder {
    new_translation(req_body)
}
