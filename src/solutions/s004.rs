use crate::solutions::solution::Solution;

pub struct S004;

const INPUT: u32 = 3;
impl Solution for S004 {
    fn execute() -> String {
        find_largest_palindrome_product(INPUT).to_string()
    }
}

fn find_largest_palindrome_product(input: u32) -> i64 {
    let mut max_palindrome = 0;
    let min = 10i64.pow(input - 1);
    let max = 10i64.pow(input) - 1;

    for i in (min..=max).rev() {
        if i * i < max_palindrome {
            break;
        }
        for j in (min..=i).rev() {
            let product = i * j;
            match product.is_palindrome() {
                true => {
                    max_palindrome = max_palindrome.max(product);
                }
                false => {
                    if product < max_palindrome {
                        break;
                    }
                }
            }
        }
    }

    max_palindrome
}

#[allow(dead_code)]
fn find_largest_palindrome_product_slow(input: u32) -> i32 {
    let mut max_palindrome = 0;
    let min = 10i32.pow(input - 1);
    let max = 10i32.pow(input) - 1;

    for i in (min..=max).rev() {
        for j in (min..=max).rev() {
            let product = i * j;
            if product.is_palindrome() {
                max_palindrome = max_palindrome.max(product);
            }
        }
    }

    max_palindrome
}

trait IsPalindrome {
    fn is_palindrome(&self) -> bool;
}

impl IsPalindrome for i32 {
    fn is_palindrome(&self) -> bool {
        let s = self.to_string();
        s.chars().zip(s.chars().rev()).all(|(a, b)| a == b)
    }
}

impl IsPalindrome for i64 {
    fn is_palindrome(&self) -> bool {
        let s = self.to_string();
        s.chars().zip(s.chars().rev()).all(|(a, b)| a == b)
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::s004::{find_largest_palindrome_product, INPUT};

    const S004_ANSWER: i64 = 906609;
    const S004_HINT: i64 = 9009;
    const HINT_INPUT: u32 = 2;

    #[test]
    fn test_find_largest_prime_factor_hint() {
        assert_eq!(S004_HINT, find_largest_palindrome_product(HINT_INPUT));
    }

    #[test]
    fn test_find_largest_prime_factor_answer() {
        assert_eq!(S004_ANSWER, find_largest_palindrome_product(INPUT));
    }
}
