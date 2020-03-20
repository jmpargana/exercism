pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: String = isbn
        .replace("-", "")
        .chars()
        .filter(|c| c.is_digit(10) || *c == 'X')
        .collect();

    !(isbn.len() != 10)
        && !(isbn.chars().enumerate().any(|(i, c)| c == 'X' && i != 9))
        && isbn
            .chars()
            .enumerate()
            .filter_map(|(i, c)| match i == 9 && c == 'X' {
                true => Some(10),
                _ => c.to_digit(10),
            })
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .fold(0, |acc, (i, e)| acc + ((10 - i) as u32 * e))
            % 11
            == 0
}
