#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Palindrome(u64, u64);

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Self {
        Palindrome(a, b)
    }

    pub fn value(&self) -> u64 {
        self.0 * self.1
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.0 = a;
        self.0 = b;
    }
}

pub fn is_palindrome(num: u64) -> bool {
    num.to_string().chars().zip(num.to_string().chars().rev()).any(|(a,b)| a!=b)
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let result: Vec<Palindrome> = (min..max).zip((min..max).rev())
        .filter(|(a,b)| is_palindrome(a*b))
        .fold(vec![], |mut data, (a,b)| {
            data.push(Palindrome(a, b));
            data
        });
    
    if result.len() < 2 { return None; }
    Some((result[0], result[result.len()-1]))
}
