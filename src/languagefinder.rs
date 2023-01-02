use translation_server_dtos_silen::{Language, TransWord};

use crate::{reasonmanagement::Reason, translateroot::TranslationError};

pub fn find_lang(id: String) -> Result<Language, TranslationError> {
    match id.as_str() {
        "en" => Ok(Language::new(
            "en",
            "english",
            vec!['.'],
            vec!['a'],
            vec![TransWord::new("", vec!["", ""])],
        )),
        "elb" => Ok(Language::new(
            "elb",
            "elbish",
            vec!['.'],
            vec!['b'],
            vec![TransWord::new("", vec!["", ""])],
        )),
        _ => Err(TranslationError {
            reason: Reason::new(
                "we do not support the language",
                &id,
                "try following languages: en, elb",
            )
            .build(),
        }),
    }
}
