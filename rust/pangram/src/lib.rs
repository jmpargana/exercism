/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let sentence = sentence
        .chars()
        .filter(|c| c.is_alphabetic() && c.is_ascii())
        .flat_map(char::to_lowercase)
        .collect::<String>();

    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .all(|c| sentence.contains(c))
}
