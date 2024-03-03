pub mod fibonacci_helper;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fibonacci_loop() {
        let result = fibonacci_helper::calculate_fibonacci_loop(4);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_calculate_fibonacci_recursive() {
        let result = fibonacci_helper::calculate_fibonacci_recursive(4);
        assert_eq!(result, 5);
    }
}
