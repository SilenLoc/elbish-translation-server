use translation_server_dtos_silen::Language;

use crate::languagefinder::find_lang;

pub struct TranslationResult {
    pub content: String,
    pub able_to_translate: bool,
}

pub struct TranslationError {
    pub reason: String,
}

pub fn translate(
    to_trans: String,
    from_lang_id: String,
    to_lang_id: String,
) -> Result<TranslationResult, TranslationError> {
    match translate_inner(to_trans, from_lang_id, to_lang_id) {
        Ok(v) => Ok(TranslationResult {
            content: (v),
            able_to_translate: (true),
        }),
        Err(e) => Err(e),
    }
}

fn translate_inner(
    to_trans: String,
    from_lang_id: String,
    to_lang_id: String,
) -> Result<String, TranslationError> {
    let lang_to_trans_from: Language = find_lang(from_lang_id)?;
    let lang_to_trans_to: Language = find_lang(to_lang_id)?;
    translator(lang_to_trans_from, lang_to_trans_to, to_trans).map(|r| r.to_content)
}

fn translator(
    lang: Language,
    target: Language,
    content: String,
) -> Result<Translation, TranslationError> {
    Ok(Translation {
        from: lang,
        to: target,
        from_content: content.clone(),
        to_content: content,
    })
}

pub struct Translation {
    pub from: Language,
    pub to: Language,
    pub from_content: String,
    pub to_content: String,
}
