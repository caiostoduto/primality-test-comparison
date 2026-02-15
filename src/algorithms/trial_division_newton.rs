pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    if n % 2 == 0 {
        return n == 2;
    }

    let mut i: u64 = 3;
    let sqrt = newton_sqrt_integer(n);

    while i <= sqrt {
        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    return true;
}

// Inspired by https://www.johndcook.com/blog/2024/01/01/computing-square-root-floor/
fn newton_sqrt_integer(n: u64) -> u64 {
    let n_u128: u128 = n as u128;

    let mut a: u128 = n_u128;
    let mut b: u128 = (n_u128 + 1) / 2;

    while b < a {
        a = b;
        b = (a * a + n_u128) / (2 * a);
    }

    a as u64
}
