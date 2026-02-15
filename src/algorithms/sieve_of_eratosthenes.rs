pub fn sieve(n: u64) -> Vec<u64> {
    if n < 2 {
        println!("There are no prime numbers less than 2.");
        return vec![];
    }

    // Bitwise storage: each u64 holds 64 bits, reducing memory by 8x
    let size = ((n as usize + 1) + 63) / 64; // ceiling division
    let mut is_prime = vec![!0u64; size]; // all bits set to 1 (true)

    // Helper functions for bit manipulation
    #[inline]
    fn get_bit(bits: &[u64], idx: usize) -> bool {
        (bits[idx / 64] & (1u64 << (idx % 64))) != 0
    }

    #[inline]
    fn clear_bit(bits: &mut [u64], idx: usize) {
        bits[idx / 64] &= !(1u64 << (idx % 64));
    }

    // Clear bits for 0 and 1
    clear_bit(&mut is_prime, 0);
    clear_bit(&mut is_prime, 1);

    let sqrt = (n as f64).sqrt() as u64;
    for i in 2..=sqrt {
        if get_bit(&is_prime, i as usize) {
            for j in (i * i..=n).step_by(i as usize) {
                clear_bit(&mut is_prime, j as usize);
            }
        }
    }

    let primes: Vec<u64> = (0..=n as usize)
        .filter(|&index| get_bit(&is_prime, index))
        .map(|index| index as u64)
        .collect();

    primes
}
