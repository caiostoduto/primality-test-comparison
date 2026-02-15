pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    if n % 2 == 0 {
        return n == 2;
    }

    let mut i: u64 = 3;

    while i <= newton_sqrt_integer(n) {
        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    return true;
}

// Inspired by https://www.johndcook.com/blog/2024/01/01/computing-square-root-floor/
fn newton_sqrt_integer(n: u64) -> u64 {
    let mut a = n;
    let mut b = (n + 1) / 2;

    while b < a {
        a = b;
        b = (a * a + n) / (2 * a);
    }

    a
}
