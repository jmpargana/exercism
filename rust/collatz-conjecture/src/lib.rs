pub fn collatz(n: u64) -> Option<u64> {
    collatz_rec(n, 0)
}

fn collatz_rec(n: u64, i: u64) -> Option<u64> {
    match (n, n % 2) {
        (0, _) => None,
        (1, _) => Some(i),
        (x, 0) => collatz_rec(x / 2, i + 1),
        (x, _) => collatz_rec(x * 3 + 1, i + 1),
    }
}
