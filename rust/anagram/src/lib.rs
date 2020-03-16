use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result = possible_anagrams.iter().cloned().collect::<HashSet<&str>>();
    result.retain(|anagram| is_anagram(word, *anagram));
    result
}

fn is_anagram(word: &str, anagram: &str) -> bool {
    anagram.to_lowercase() != word.to_lowercase() && char_count(anagram) == char_count(word)
}

fn char_count(word: &str) -> HashMap<char, u32> {
    word.to_lowercase()
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
}
