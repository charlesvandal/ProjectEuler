use crate::solutions::solution::{Solution, SolutionResult};
use primes::primes_helper;
use std::time::Instant;

const INPUT: i64 = 600851475143;

pub struct S003;

impl Solution for S003 {
    fn run() -> SolutionResult {
        let start = Instant::now();
        let result = find_largest_prime_factor(INPUT);
        let end = Instant::now();
        SolutionResult::new(end - start, result.to_string())
    }
}

fn find_largest_prime_factor(number: i64) -> i64 {
    let upperbound = (number as f64).sqrt() as i64;

    let primes = primes_helper::sieve(upperbound);
    for iter in primes.iter().rev() {
        if number % *iter == 0 {
            return *iter;
        }
    }
    return 0;
}

#[cfg(test)]
mod test {
    use crate::solutions::s003;
    use crate::solutions::s003::INPUT;

    const S003_ANSWER: i64 = 6857;
    const S003_HINT: i64 = 29;
    const HINT_INPUT: i64 = 13195;

    #[test]
    fn test_find_largest_prime_factor_hint() {
        assert_eq!(S003_HINT, s003::find_largest_prime_factor(HINT_INPUT));
    }

    #[test]
    fn test_find_largest_prime_factor_answer() {
        assert_eq!(S003_ANSWER, s003::find_largest_prime_factor(INPUT));
    }
}