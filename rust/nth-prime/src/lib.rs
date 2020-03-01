pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut counter = 3;
    let mut flag = true;

    while primes.len() <= n as usize {
        for prime in &primes {
            if counter % prime == 0 {
                flag = false;
                break
            }
        }
        if flag == true {
            primes.push(counter);
        }   
        flag = true; 
        counter += 1; 
    }
    
    primes[n as usize]
}
