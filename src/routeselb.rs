use actix_web::{get, post, HttpResponse, Responder};

use crate::{dtos::TransReq, jsontransreqhandler::handle_trans_req};

#[post("/translate")]
pub async fn translate(req_body: String) -> impl Responder {
    handle_trans_req(req_body)
}

#[get("/example")]
pub async fn example() -> impl Responder {
    HttpResponse::Ok().body(TransReq::example())
}

#[get("/ready")]
pub async fn ready() -> impl Responder {
    HttpResponse::Ok().body("ready to response")
}
