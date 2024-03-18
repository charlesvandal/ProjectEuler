pub fn sieve(upperbound: i64) -> Vec<i64> {
    if upperbound < 2 {
        return Vec::new();
    }
    if upperbound == 2 {
        vec![2];
    }

    let odd_number_maximum = ((upperbound as f64 + 1.0) / 2.0) as usize;
    let maximum_number_to_check = (((upperbound as f64).sqrt().floor() - 1.0) / 2.0) as usize;
    let mut is_prime_array: Vec<bool> = vec![true; odd_number_maximum];
    let mut primes: Vec<i64> = vec![2];
    is_prime_array[0] = false;

    for i in 0..=maximum_number_to_check {
        if is_prime_array[i] {
            for j in (2 * i * (i + 1)..odd_number_maximum).step_by(2 * (i) + 1) {
                is_prime_array[j] = false;
            }
        }
    }

    for (index, is_prime) in is_prime_array.iter().enumerate() {
        if *is_prime {
            primes.push(2 * (index as i64) + 1);
        }
    }

    return primes;
}