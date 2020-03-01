pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes = (2..upper_bound + 1).collect::<Vec<_>>();

    for p in 2..(upper_bound + 1) {
        primes.retain(|&x| x <= p || x % p != 0);
    }
    primes
}
