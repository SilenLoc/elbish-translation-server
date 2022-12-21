use serde::Deserialize;
use serde::Serialize;

use crate::translateroot::TranslationResult;

#[derive(Serialize, Deserialize)]
pub struct TransReq {
    pub content: String,
}

impl TransReq {
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
