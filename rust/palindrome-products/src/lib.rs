use itertools::*;

// #[derive(Debug, PartialEq, Eq)]
// pub struct Palindrome {
//     value: u64,
//     factors: Vec<(u64, u64)>
// }

// impl Palindrome {
//     pub fn new(a: u64, b: u64) -> Palindrome {
//         Palindrome {
//             value: a * b,
//             factors: vec![(a, b)]
//         }
//     }

//     pub fn value(&self) -> u64 {
//         self.value
//     }

//     pub fn insert(&mut self, a: u64, b: u64) {
//         match a * b {
//             x if x == self.value && !self.factors.contains(&(a, b)) => 
//                 self.factors.push((a, b)),
//             _ => ()
//         }
//     }
// }


pub type Palindrome = u64;


pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let result: Vec<Palindrome> = iproduct!(min..=max, min..=max)
        .map(|(i, j)| i * j)
        .filter(|p| is_palindrome(*p))
        .collect();

    match result.len() {
        0 => None,
        _ => Some((result.iter().min().cloned().unwrap(), result.iter().max().cloned().unwrap()))
    }
    // let mut palindromes = Vec::new();

    // for i in min..=max {
    //     for j in min..=max {
    //         if is_palindrome(i * j) {
    //             palindromes.push(Palindrome::new(i, j));
    //         }
    //     }
    // }

    // match palindromes.len() {
    //     0 => None,
    //     _ => Some((palindromes[0], palindromes[palindromes.len()-1]))
    // }
}

pub fn is_palindrome(product: u64) -> bool {
    let num = product.to_string();
    num == num.chars().rev().collect::<String>()
}
