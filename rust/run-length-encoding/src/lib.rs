pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut start = String::from(source);

    while start.len() > 0 {
        let c = start.chars().next().unwrap();
        let pos = match start.find(|x| x != c) {
            None => start.len(),
            Some(y) => y,
        };

        match pos {
            1 => result.push(c),
            _ => result.push_str(&format!("{}{}", pos, c))
        }
        start = start.split_off(pos);
    }
    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut digit = String::new();

    for c in source.chars() {
        if c.is_numeric() { digit.push(c) }
        else {
            if let Ok(amount) = digit.parse() {
                result.push_str(&c.to_string().repeat(amount));
                digit = String::new();
            } else { result.push(c); }
        }
    }
    result
}
