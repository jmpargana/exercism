pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len { return vec![] }
    
    digits.chars().collect::<Vec<char>>().windows(len).map(|x| x.iter().collect())
        .collect::<Vec<String>>()
}
