pub fn score(word: &str) -> u64 {
    word.chars()
        .map(|c| c.to_ascii_lowercase())
        .map(|c| 
            match c {
            _ if "aeioulnrst".contains(c) => 1,
            _ if "bcmp".contains(c) => 3,
            _ if "fhvwy".contains(c) => 4,
            'k' => 5,
            'd' | 'g' => 2,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0,
        }).sum()
}
