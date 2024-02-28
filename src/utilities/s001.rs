use std::time::{Duration, Instant};

pub fn run() {
    let upper_bound = 1000;
    let multiple_a = 3;
    let multiple_b = 5;

    get_sum_of_multiples_slow(multiple_a, multiple_b, upper_bound);
    get_sum_of_multiples_fast(multiple_a, multiple_b, upper_bound);
}

fn get_sum_of_multiples_slow(a:i64, b:i64, upper_bound:i64) {
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
}

fn get_sum_of_multiples_fast(a:i64, b:i64, upper_bound:i64) {
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
}

fn get_number_divisible_by_n(upper_bound:i64, quotien:i64) -> i64{
    return (upper_bound - 1) / quotien
}

fn print_execution_time(sum:i64, execution_time:Duration ) {
    println!("{} -> execution time (s): {:?}", sum, execution_time);
}