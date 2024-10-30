use crate::solutions::solution::{Solution, SolutionResult};
use fibonacci::fibonacci_helper::{calculate_fibonacci_loop, calculate_fibonacci_recursive};
use std::time::Instant;

const NUMBER_LIMIT: i64 = 4000000;

pub struct S002;

impl Solution for S002 {
    fn run() -> SolutionResult {
        let start = Instant::now();
        let result = sum_even_valued_fibonacci_terms_loop();
        let end = Instant::now();
        SolutionResult::new(end - start, result.to_string())
    }
}

fn sum_even_valued_fibonacci_terms_loop() -> i64 {
    let mut sum = 0;
    let mut index = 2;

    while calculate_fibonacci_loop(index) <= NUMBER_LIMIT {
        if calculate_fibonacci_loop(index) % 2 == 0
        {
            sum += calculate_fibonacci_loop(index);
        }

        index += 1;
    }

    sum
}

fn sum_even_valued_fibonacci_terms_recursive() -> i64 {
    let mut sum = 0;
    let mut index = 2;

    while calculate_fibonacci_recursive(index) <= NUMBER_LIMIT {
        if calculate_fibonacci_recursive(index) % 2 == 0
        {
            sum += calculate_fibonacci_recursive(index);
        }

        index += 1;
    }

    sum
}

#[cfg(test)]
mod test {
    use crate::solutions::s002;

    const S002_ANSWER: i64 = 4613732;

    #[test]
    fn test_sum_even_valued_fibonacci_terms_loop() {
        assert_eq!(S002_ANSWER, s002::sum_even_valued_fibonacci_terms_loop());
    }

    #[test]
    fn test_sum_even_valued_fibonacci_terms_recursive() {
        assert_eq!(S002_ANSWER, s002::sum_even_valued_fibonacci_terms_loop());
    }
}