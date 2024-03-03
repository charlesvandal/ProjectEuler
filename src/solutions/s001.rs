use std::time::{Duration, Instant};
use crate::utilities::defines::solution_runner_defines;

const UPPER_BOUND: i64 = 1000;
const MULTIPLE_A: i64 = 3;
const MULTIPLE_B: i64 = 5;

pub fn run() -> bool{
    get_sum_of_multiples_slow(MULTIPLE_A, MULTIPLE_B, UPPER_BOUND);
    get_sum_of_multiples_fast(MULTIPLE_A, MULTIPLE_B, UPPER_BOUND);

    solution_runner_defines::SUCCESS
}

fn get_sum_of_multiples_slow(a:i64, b:i64, upper_bound:i64) -> i64{
    let start = Instant::now();
    let mut sum = 0;

	for i in 1..upper_bound {
        if i % a == 0 || i % b == 0
		{
			sum += i;
		}
    }

    let end = Instant::now();
    let execution_time  = end - start;

    print_execution_time(sum, execution_time);

    sum
}

fn get_sum_of_multiples_fast(a:i64, b:i64, upper_bound:i64) -> i64{
    let start = Instant::now();
    let count_divisible_by_a = get_number_divisible_by_n(upper_bound, a);
    let count_divisible_by_b = get_number_divisible_by_n(upper_bound, b);
    let count_divisible_by_ab = get_number_divisible_by_n(upper_bound, a*b);

    let sum_divisible_by_a = (a * count_divisible_by_a * (count_divisible_by_a + 1)) / 2;
    let sum_divisible_by_b = (b * count_divisible_by_b * (count_divisible_by_b + 1)) / 2;
    let sum_divisible_by_ab = (a * b * count_divisible_by_ab * (count_divisible_by_ab + 1)) / 2;

    let sum = sum_divisible_by_a + sum_divisible_by_b - sum_divisible_by_ab;

    let end = Instant::now();
    let execution_time: Duration  = end - start;

    print_execution_time(sum, execution_time);

    sum
}

fn get_number_divisible_by_n(upper_bound:i64, quotien:i64) -> i64{
    (upper_bound - 1) / quotien
}

fn print_execution_time(sum:i64, execution_time:Duration ) {
    println!("{} -> execution time (s): {:?}", sum, execution_time);
}

#[cfg(test)]
mod test {
    use crate::solutions::s001::*;

    const S001_ANSWER: i64 = 233168;

    #[test]
    fn test_get_sum_of_multiples_slow() {
        let sum = get_sum_of_multiples_slow(MULTIPLE_A, MULTIPLE_B, UPPER_BOUND);

        assert_eq!(sum, S001_ANSWER)
    }

    #[test]
    fn test_get_sum_of_multiples_fast() {
        let sum = get_sum_of_multiples_fast(MULTIPLE_A, MULTIPLE_B, UPPER_BOUND);

        assert_eq!(sum, S001_ANSWER)
    }
}