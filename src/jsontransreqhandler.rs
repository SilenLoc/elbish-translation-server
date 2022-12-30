use actix_web::HttpResponse;
use translation_server_dtos_silen::{JsonErr, TransErr, TransReq, TransResponse};

use crate::translateroot::{translate, TranslationResult};

pub fn handle_trans_req(req_body: String) -> HttpResponse {
    let extracted: Result<TransReq, JsonErr> = safe_unwrap_json(req_body);

    let safe = match extracted {
        Ok(v) => v,
        Err(e) => return HttpResponse::BadRequest().body(e.content),
    };

    let translation_result = translate_content(&safe);

    let answer = translation_result
        .map(|r| serde_json::to_string(&r).unwrap())
        .map_err(|e| serde_json::to_string(&e).unwrap());

    match answer {
        Ok(t) => HttpResponse::Ok().body(t),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

fn safe_unwrap_json(req_body: String) -> Result<TransReq, JsonErr> {
    let result: Result<TransReq, JsonErr> =
        serde_json::from_str(&req_body).map_err(|err| JsonErr::new(&err.to_string()));
    result
}

fn translate_content(req: &TransReq) -> Result<TransResponse, TransErr> {
    match req.content.as_str() {
        "" => Err(TransErr::new("Blank translation content")),
        _ => Ok(TransResponse::from_result(translate(
            req.content.to_string(),
            req.from.to_string(),
            req.to.to_string(),
        ))),
    }
}

trait TransResponseFrom {
    fn from_result(result: TranslationResult) -> TransResponse;
}

impl TransResponseFrom for TransResponse {
    fn from_result(result: TranslationResult) -> TransResponse {
        TransResponse {
            content: result.content,
            able_to_translate: result.able_to_translate,
        }
    }
}
