/// Add two integers
///
/// Args:
///     x: an integer to sum
///     y: an integer to sum
///
/// Returns:
///     the sum of the integers
pub fn sum(x: usize, y: usize) -> usize {
    x + y
}

/// Sum all numbers from 0..range
///
/// Args:
///     range: upper bound to loop until
///
/// Returns:
///     total: the sum of the looped over values
pub fn adder(range: usize) -> usize {
    let mut total = 0;
    for i in 0..range {
        total += i
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn one_and_one_is_two() {
        let result = sum(1, 1);
        assert_eq!(result, 2);
    }

    #[test]
    fn first_ten_sum() {
        let result = adder(10);
        assert_eq!(result, 45);
    }
}
