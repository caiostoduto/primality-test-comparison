pub fn sieve(n: u64) -> Vec<u64> {
    if n < 2 {
        println!("There are no prime numbers less than 2.");
        return vec![];
    }

    // vec [0, n]
    let mut is_prime = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let sqrt = (n as f64).sqrt() as u64;
    for i in 2..=sqrt {
        if is_prime[i as usize] {
            for j in (i * i..=n).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }

    let primes: Vec<u64> = is_prime
        .iter()
        .enumerate()
        .filter_map(|(index, &prime)| if prime { Some(index as u64) } else { None })
        .collect();

    primes
}
