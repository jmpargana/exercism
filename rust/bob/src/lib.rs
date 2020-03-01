pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.ends_with('?') && is_yell(&m) => 
            "Calm down, I know what I'm doing!",
        m if is_yell(&m) =>
            "Whoa, chill out!",
        m if m.ends_with('?') =>
            "Sure.",
        m if m == "" =>
            "Fine. Be that way!",
        _ => "Whatever."
    }
}

pub fn is_yell(message: &str) -> bool {
    message.to_uppercase() == message && message.chars().filter(|x| x.is_alphabetic()).count() > 0
}
