use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    words.to_lowercase()
         .split(|c: char| !c.is_alphanumeric() && c != '\'')
         .map(|word| word
             .trim_matches(|c: char| c == '\'' && word.matches('\'').count() == 2))
         .filter(|w| !w.is_empty())
         .fold(HashMap::new(), |mut acc, word| {
             *acc.entry(word.to_string()).or_insert(0) += 1; acc
    })
}
