pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut start = 2u64;
    let mut num = n;

    while start <= num {
        if num % start == 0 {
            factors.push(start);
            num /= start;
        } else {
            start += 1;
        }
    }

    factors
}
