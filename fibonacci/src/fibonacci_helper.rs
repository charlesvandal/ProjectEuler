pub fn calculate_fibonacci_recursive(index:i64) -> i64 {
    if index <= 2 {
		return index;
	}

	return calculate_fibonacci_recursive(index - 1) + calculate_fibonacci_recursive(index - 2);
}

pub fn calculate_fibonacci_loop(index:i64) -> i64 {
    if index <= 2 {
		return index;
	}

    let mut f1 = 1;
    let mut f2 = 2;
    let mut f3 = 3;

    for _ in 3..=index {
        f3 = f1 + f2;
        [f1, f2] = [f2, f3];
    }

	f3
}