pub struct TranslationResult {
    pub content: String,
    pub able_to_translate: bool,
}

struct TranslationError {}

pub fn translate(to_trans: String) -> TranslationResult {
    match translate_inner(to_trans) {
        Ok(v) => TranslationResult {
            content: (v),
            able_to_translate: (true),
        },
        Err(_) => TranslationResult {
            content: ("".to_string()),
            able_to_translate: (false),
        },
    }
}

fn translate_inner(to_trans: String) -> Result<String, TranslationError> {
    Ok(to_trans)
}
