pub fn encode(n: u64) -> String {
    let segments = group(n);

    const POWERS: [&str; 7] = [" quintillion", " quadrillion", 
                               " trillion", " billion",
                               " million", " thousand", ""];

    if n == 0 { return "zero".to_string() }
    
    segments.iter().enumerate()
        .filter(|(_,b)| **b != 0)
        .fold(String::new(), |mut acc, (i, val)| {
            if !acc.is_empty() { acc += " "; }

            acc += &hundreds(*val);
            acc += POWERS[POWERS.len()-segments.len()+i];
            acc
    })
}

pub fn digits(n: u64) -> String {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => "",
    }.to_string()
}

pub fn teens(n: u64) -> String {
    match n {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    }.to_string()
}

pub fn decimals(n: u64) -> String {
    let temp = match n {
        1..=9 => return digits(n),
        10..=19 => return teens(n),
        20..=29 => "twenty",
        30..=39 => "thirty",
        40..=49 => "forty",
        50..=59 => "fifty",
        60..=69 => "sixty",
        70..=79 => "seventy",
        80..=89 => "eighty",
        90..=99 => "ninety",
        _ => "",
    };

    if n % 10 != 0 { return format!("{}-{}", temp, digits(n%10)) }
    temp.to_string()
}

pub fn hundreds(n: u64) -> String {
    match n {
        n if n % 100 == 0 && n / 100 != 0 => format!("{} {}", 
                    digits(n/100), 
                    "hundred".to_string()),
        0..=99 => decimals(n%100),
        _ => format!("{} {} {}", 
                    digits(n/100), 
                    "hundred".to_string(), 
                    decimals(n%100))       
    }
}

pub fn group(n: u64) -> Vec<u64> {
    let mut n = n;  // make mutable
    let mut result: Vec<u64> = Vec::new();

    while n > 0 {
        result.push(n % 1000);
        n /= 1000;
    }
    result.iter().rev().map(|x| *x).collect()
}
