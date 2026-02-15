/// Miller-Rabin primality test
/// This is a probabilistic primality test, but for u64 values we use
/// a deterministic set of witnesses that guarantees correctness.

pub fn is_prime(n: u64) -> bool {
    // Handle small cases
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    if n < 9 {
        return true; // 5 and 7
    }

    // For u64, these witnesses are sufficient for deterministic results
    // This set is proven to work correctly for all n < 2^64
    let witnesses: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    // Decompose n-1 = 2^r * d once, shared across all witnesses
    let mut d = n - 1;
    let mut r = 0u32;
    while d & 1 == 0 {
        d >>= 1;
        r += 1;
    }

    for &witness in &witnesses {
        if n == witness {
            return true;
        }
        if !miller_rabin_test(n, witness, d, r) {
            return false;
        }
    }

    true
}

/// Performs modular exponentiation: (base^exp) mod m
/// Uses u128 intermediates to avoid overflow for u64 inputs.
#[inline]
fn mod_pow(base: u64, mut exp: u64, m: u64) -> u64 {
    if m == 1 {
        return 0;
    }

    let m128 = m as u128;
    let mut base128 = (base % m) as u128;
    let mut result: u128 = 1;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base128) % m128;
        }
        exp >>= 1;
        base128 = (base128 * base128) % m128;
    }

    result as u64
}

/// Squaring modulo n using u128 to avoid overflow.
#[inline(always)]
fn mod_sqr(x: u64, n: u64) -> u64 {
    let x128 = x as u128;
    let n128 = n as u128;
    ((x128 * x128) % n128) as u64
}

/// Performs one round of the Miller-Rabin test with a given witness.
/// Takes pre-computed d and r where n-1 = 2^r * d.
#[inline]
fn miller_rabin_test(n: u64, witness: u64, d: u64, r: u32) -> bool {
    // Compute x = witness^d mod n
    let mut x = mod_pow(witness, d, n);

    if x == 1 || x == n - 1 {
        return true;
    }

    // Square x (r-1) times
    for _ in 0..r - 1 {
        x = mod_sqr(x, n);
        if x == n - 1 {
            return true;
        }
    }

    false
}
