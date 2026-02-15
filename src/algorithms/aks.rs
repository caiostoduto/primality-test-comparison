// AKS (Agrawal-Kayal-Saxena) Primality Test
// A deterministic polynomial-time primality test

pub fn is_prime(n: u64) -> bool {
    // Step 1: Check if n is a perfect power (n = a^b for b > 1)
    if is_perfect_power(n) {
        return false;
    }

    // Step 2: Find the smallest r such that ord_r(n) > log²(n)
    let r = find_smallest_r(n);

    // Step 3: Check if 1 < gcd(a, n) < n for all a ≤ r
    for a in 2..=r.min(n - 1) {
        let g = gcd(a, n);
        if g > 1 && g < n {
            return false;
        }
    }

    // Step 4: If n ≤ r, we're done
    if n <= r {
        return true;
    }

    // Step 5: Polynomial congruence test
    // For a = 1 to floor(sqrt(φ(r)) * log2(n))
    let limit = ((euler_phi(r) as f64).sqrt() * (n as f64).log2()).floor() as u64;

    for a in 1..=limit {
        if !check_polynomial_congruence(n, r, a) {
            return false;
        }
    }

    true
}

// Check if n is a perfect power
fn is_perfect_power(n: u64) -> bool {
    if n == 1 {
        return true;
    }

    // Check for each possible exponent b from 2 to log2(n)
    let max_b = (n as f64).log2() as u32;

    for b in 2..=max_b {
        let a = (n as f64).powf(1.0 / b as f64).round() as u64;

        // Check a and a±1 to account for floating point errors
        for candidate in [a.saturating_sub(1), a, a.saturating_add(1)] {
            if candidate > 1 && pow_checked(candidate, b) == Some(n) {
                return true;
            }
        }
    }

    false
}

// Compute a^b with overflow checking
fn pow_checked(mut base: u64, mut exp: u32) -> Option<u64> {
    let mut result = 1u64;

    while exp > 0 {
        if exp % 2 == 1 {
            result = result.checked_mul(base)?;
        }
        exp /= 2;
        if exp > 0 {
            base = base.checked_mul(base)?;
        }
    }

    Some(result)
}

// Find smallest r such that ord_r(n) > log²(n)
fn find_smallest_r(n: u64) -> u64 {
    let log_n_sq = ((n as f64).log2().powi(2)).ceil() as u64;

    for r in 2.. {
        if gcd(n, r) != 1 {
            continue;
        }

        // Compute the order of n modulo r
        let order = multiplicative_order(n, r);

        if order > log_n_sq {
            return r;
        }
    }

    unreachable!()
}

// Compute the multiplicative order of n modulo r
fn multiplicative_order(n: u64, r: u64) -> u64 {
    let n_mod = n % r;
    let mut current = n_mod;

    for k in 1..=r {
        if current == 1 {
            return k;
        }
        current = (current * n_mod) % r;
    }

    r
}

// Compute Euler's totient function φ(n)
fn euler_phi(n: u64) -> u64 {
    let mut result = n;
    let mut n_mut = n;
    let mut p = 2;

    while p * p <= n_mut {
        if n_mut % p == 0 {
            while n_mut % p == 0 {
                n_mut /= p;
            }
            result -= result / p;
        }
        p += if p == 2 { 1 } else { 2 };
    }

    if n_mut > 1 {
        result -= result / n_mut;
    }

    result
}

// Greatest common divisor
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Check polynomial congruence: (X + a)^n ≡ X^n + a (mod X^r - 1, n)
fn check_polynomial_congruence(n: u64, r: u64, a: u64) -> bool {
    // Represent polynomial as coefficients array
    // We only need to track coefficients modulo n and degree modulo r

    let r_usize = r as usize;
    let mut poly = vec![0u64; r_usize];

    // Start with (X + a)
    poly[0] = a % n;
    poly[1] = 1;

    // Compute (X + a)^n mod (X^r - 1, n) using binary exponentiation
    let result = poly_pow_mod(&poly, n, r, n);

    // Check if result equals X^n + a mod (X^r - 1, n)
    let n_mod_r = (n % r) as usize;
    let expected_a = a % n;

    for i in 0..r_usize {
        let expected = if i == 0 {
            expected_a
        } else if i == n_mod_r {
            1
        } else {
            0
        };

        if result[i] != expected {
            return false;
        }
    }

    true
}

// Polynomial exponentiation: poly^exp mod (X^r - 1, n)
fn poly_pow_mod(poly: &[u64], mut exp: u64, r: u64, n: u64) -> Vec<u64> {
    let r_usize = r as usize;
    let mut result = vec![0u64; r_usize];
    result[0] = 1; // Start with polynomial 1

    let mut base = poly.to_vec();

    while exp > 0 {
        if exp % 2 == 1 {
            result = poly_mul_mod(&result, &base, r, n);
        }
        exp /= 2;
        if exp > 0 {
            base = poly_mul_mod(&base, &base, r, n);
        }
    }

    result
}

// Polynomial multiplication modulo (X^r - 1, n)
fn poly_mul_mod(a: &[u64], b: &[u64], r: u64, n: u64) -> Vec<u64> {
    let r_usize = r as usize;
    let mut result = vec![0u64; r_usize];

    for i in 0..r_usize {
        for j in 0..r_usize {
            if a[i] == 0 || b[j] == 0 {
                continue;
            }

            let coeff = ((a[i] as u128 * b[j] as u128) % n as u128) as u64;
            let pos = (i + j) % r_usize;

            result[pos] = (result[pos] + coeff) % n;
        }
    }

    result
}
