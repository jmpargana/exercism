pub fn is_armstrong_number(num: u32) -> bool {
    const RADIX: u32 = 10;
    let s = num.to_string();
    let result: u32 = s.chars().map(|x| x.to_digit(RADIX).unwrap().pow(s.len() as u32)).sum();
    result == num
}
