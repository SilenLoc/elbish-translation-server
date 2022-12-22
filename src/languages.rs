use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Language {
    pub id: String,
    pub label: String,
    pub unit_split_symbols: Vec<char>,
    pub allowed_symbols: Vec<char>,
    pub words: Vec<TransWord>,
}

impl Language {
    pub fn new(
        id: &str,
        label: &str,
        unit_split_symbols: Vec<char>,
        allowed_symbols: Vec<char>,
        _words: Vec<TransWord>,
    ) -> Language {
        Language {
            id: (id.to_string()),
            label: (label.to_string()),
            unit_split_symbols: (unit_split_symbols),
            allowed_symbols: (allowed_symbols),
            words: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransWord {
    pub symbol_chain: Vec<char>,
    pub meanings: Vec<WordToOtherLang>,
}

impl TransWord {
    pub fn new(word: &str, meanings: Vec<&str>) -> TransWord {
        TransWord {
            symbol_chain: word.chars().collect(),
            meanings: meanings
                .into_iter()
                .map(|meaning| WordToOtherLang::new(meaning.chars().collect()))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WordToOtherLang {
    pub symbol_chain: Vec<char>,
}
impl WordToOtherLang {
    pub fn new(symbol_chain: Vec<char>) -> WordToOtherLang {
        WordToOtherLang { symbol_chain }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::languages::{Language, TransWord};

    #[test]
    fn init_languages() {
        let english = Language::new(
            "en",
            "english",
            vec!['.'],
            vec!['a'],
            vec![TransWord::new("", vec!["", ""])],
        );
        assert_eq!(english.id, "en".to_string());
    }

    #[test]
    fn init_new_word() {
        let trans_word = TransWord::new("hello", vec!["hello"]);
        assert_eq!(
            trans_word.meanings[0].symbol_chain,
            vec!['h', 'e', 'l', 'l', 'o']
        );
    }
}
