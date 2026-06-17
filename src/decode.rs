use crate::utils::alphabet::DECODE_MAP;

pub fn decode(value: String) -> String {
    let words: Vec<&str> = value.split('/').collect();
    let mut result_words: Vec<String> = Vec::new();

    for word in words {
        let mut letters: Vec<char> = Vec::new();

        for code in word.trim().split_whitespace() {
            if let Some(c) = DECODE_MAP.get(code) {
                letters.push(*c);
            }
        }

        let decoded_word: String = letters.into_iter().collect();
        result_words.push(decoded_word);
    }

    result_words.join(" ")
}