pub mod primes_helper;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_prime_sieve() {
        let result = primes_helper::sieve(15);
        assert_eq!(result, vec![2, 3, 5, 7, 11, 13]);
    }

    #[test]
    fn test_calculate_prime_sieve_with_2_returns_2() {
        let result = primes_helper::sieve(2);
        assert_eq!(result, vec![2]);
    }

    #[test]
    fn test_calculate_prime_sieve_with_negative_value_returns_empty() {
        let result = primes_helper::sieve(-5);
        assert_eq!(result, vec![]);
    }
}