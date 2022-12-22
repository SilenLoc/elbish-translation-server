use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use actix_web::HttpResponse;

use crate::dtos::{JsonErr, NewTransErr, NewTransReq};

pub fn new_translation(req_body: String) -> HttpResponse {
    let extracted: Result<NewTransReq, JsonErr> = safe_unwrap_json(req_body);

    let safe = match extracted {
        Ok(v) => v,
        Err(e) => return HttpResponse::BadRequest().body(e.content),
    };

    let new_translation_result = inner_new_translation(
        &safe.from_lang,
        &safe.to_lang,
        &safe.word,
        safe.meanings.iter().map(|s| s.as_ref()).collect(),
    );

    match new_translation_result {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => HttpResponse::BadRequest().body(e.content),
    }
}

fn inner_new_translation(
    from_lang_id: &str,
    to_lang_id: &str,
    word: &str,
    meanings: Vec<&str>,
) -> Result<String, NewTransErr> {
    match from_lang_id {
        "" => Err(NewTransErr::new("from language can not be blank")),
        _ => Ok(new_translator(from_lang_id, to_lang_id, word, meanings)),
    }
}

fn new_translator(from_lang_id: &str, to_lang_id: &str, word: &str, meanings: Vec<&str>) -> String {
    save_translation(from_lang_id, to_lang_id, word, meanings);
    "thank you for contributing".to_string()
}

fn safe_unwrap_json(req_body: String) -> Result<NewTransReq, JsonErr> {
    let result: Result<NewTransReq, JsonErr> =
        serde_json::from_str(&req_body).map_err(|err| JsonErr::new(&err.to_string()));
    result
}

fn save_translation(from_lang_id: &str, to_lang_id: &str, word: &str, meanings: Vec<&str>) {
    let to_save = word.to_string() + "=" + &meanings.concat() + ">";

    let target_path = &(from_lang_id.to_owned() + "_" + to_lang_id);
    if Path::new(target_path).exists() {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(target_path)
            .unwrap();
        let _ = file.write_all(to_save.as_str().as_bytes());
    } else {
        let mut file = File::create(target_path).unwrap();
        let _ = file.write_all(to_save.as_str().as_bytes());
    }
}
