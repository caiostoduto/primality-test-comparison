pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    if n % 2 == 0 {
        return n == 2;
    }

    let mut i: u64 = 3;
    let sqrt = (n as f64).sqrt() as u64;

    while i <= sqrt {
        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    return true;
}
