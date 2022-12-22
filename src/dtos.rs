use serde::Deserialize;
use serde::Serialize;

use crate::translateroot::TranslationResult;

#[derive(Serialize, Deserialize)]
pub struct TransReq {
    pub content: String,
    pub from: String,
    pub to: String,
}

impl TransReq {
    pub fn new(content: &str, from: &str, to: &str) -> TransReq {
        TransReq {
            content: content.to_string(),
            from: from.to_string(),
            to: to.to_string(),
        }
    }

    pub fn example() -> String {
        serde_json::to_string(&TransReq::new("content to translate", "en", "elb")).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransResponse {
    pub content: String,
    pub able_to_translate: bool,
}

impl TransResponse {
    pub fn from(result: TranslationResult) -> TransResponse {
        TransResponse {
            content: result.content,
            able_to_translate: result.able_to_translate,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransErr {
    pub content: String,
}

impl TransErr {
    pub fn new(content: &str) -> TransErr {
        TransErr {
            content: content.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonErr {
    pub content: String,
}

impl JsonErr {
    pub fn new(content: &str) -> JsonErr {
        JsonErr {
            content: content.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewTransReq {
    pub from_lang: String,
    pub to_lang: String,
    pub word: String,
    pub meanings: Vec<String>,
}

impl NewTransReq {
    pub fn new(from_lang: &str, to_lang: &str, word: &str, meanings: Vec<&str>) -> NewTransReq {
        NewTransReq {
            from_lang: from_lang.to_string(),
            to_lang: to_lang.to_string(),
            word: word.to_string(),
            meanings: meanings.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn example() -> String {
        serde_json::to_string(&NewTransReq::new("en", "elb", "helllo", vec!["something"])).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewTransErr {
    pub content: String,
}

impl NewTransErr {
    pub fn new(content: &str) -> NewTransErr {
        NewTransErr {
            content: content.to_string(),
        }
    }
}
