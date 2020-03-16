static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn encode(plain: &str) -> String {
    plain
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(swap)
        .enumerate()
        .map(|(i, c)| match (i + 1) % 5 {
            0 => format!("{} ", c),
            _ => c.to_string(),
        })
        .collect::<String>()
        .trim()
        .to_string()
}

pub fn decode(cipher: &str) -> String {
    cipher
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(swap)
        .collect()
}

fn swap(c: char) -> char {
    ALPHABET
        .find(c)
        .map_or(c, |i| ALPHABET.chars().nth(26 - i - 1).unwrap())
}
