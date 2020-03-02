use std::panic::catch_unwind;

pub fn answer(command: &str) -> Option<i32> {
    let parsed = parser(command);

    if parsed.len() % 2 == 1 {
        match catch_unwind(|| calculate(parsed)) {
            Ok(res) => return Some(res),
            Err(_) => return None,
        }
    }
    None
}

pub fn parser(command: &str) -> Vec<String> {
    command
        .trim_start_matches("What is ")
        .trim_end_matches("?")
        .replace("th power", "")
        .replace("nd power", "")
        .replace("rd power", "")
        .split_whitespace()
        .filter(|w| w != &"by")
        .map(|w| w.to_string())
        .collect()
}

fn calculate(operations: Vec<String>) -> i32 {
    let num_iter = operations.iter().skip(2).step_by(2).map(|w| parse_num(w));

    let op_iter = operations.iter().skip(1).step_by(2);

    num_iter.zip(op_iter).fold(
        parse_num(operations.first().unwrap()),
        |acc, (n, op)| match &op[..] {
            "plus" => acc + n,
            "minus" => acc - n,
            "multiplied" => acc * n,
            "divided" => acc / n,
            "raised" => acc.pow(n as u32),
            _ => panic!("Invalid input!"),
        },
    )
}

fn parse_num(w: &str) -> i32 {
    w.parse::<i32>().unwrap()
}
