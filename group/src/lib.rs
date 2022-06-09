/// Add two integers
pub fn sum(x: usize, y: usize) -> usize {
    x + y
}


#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn one_and_one_is_two() {
        let result = sum(1, 1);
        assert_eq!(result, 2);
    }
}
