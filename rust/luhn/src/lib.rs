pub fn is_valid(code: &str) -> bool {
    !(code.chars().any(|c| !(c.is_digit(10) || c.is_whitespace())))
        && !(code
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>()
            .len()
            == 1)
        && code
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, e)| {
                let v = if i % 2 == 0 || e == 9 { e } else { (e * 2) % 9 };
                acc + v
            })
            % 10
            == 0
}
