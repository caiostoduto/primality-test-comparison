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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newton_sqrt() {
        assert_eq!(newton_sqrt_integer(4), 2);
        assert_eq!(newton_sqrt_integer(9), 3);
        assert_eq!(newton_sqrt_integer(16), 4);
        assert_eq!(newton_sqrt_integer(100), 10);
        assert_eq!(newton_sqrt_integer(101), 10);
    }

    #[test]
    fn test_edge_cases() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
    }

    #[test]
    fn test_small_primes() {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        for &p in &primes {
            assert!(is_prime(p));
        }
    }

    #[test]
    fn test_small_composites() {
        let composites = [4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25];
        for &c in &composites {
            assert!(!is_prime(c));
        }
    }

    #[test]
    fn test_larger_primes() {
        let primes = [97, 541, 7919, 104729];
        for &p in &primes {
            assert!(is_prime(p));
        }
    }

    #[test]
    fn test_larger_composites() {
        let composites = [100, 1000, 10000, 52939758, 1029105];
        for &c in &composites {
            assert!(!is_prime(c));
        }
    }

    #[test]
    fn test_large_primes() {
        assert!(is_prime(2147483647)); // 2^31 - 1 (Mersenne prime)
        assert!(is_prime(4294967291)); // Largest prime < 2^32
    }
}
