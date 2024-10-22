pub fn sum_of_squares(first_n: i64) -> i64 {
    (1..=first_n).reduce(|acc, x| acc + (x * x)).unwrap()
}

pub fn square_of_sums(first_n: i64) -> i64 {
    let sums = (1..=first_n).reduce(|acc, x| acc + x).unwrap();
    sums * sums
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_example_sum_of_squares() {
        let actual = sum_of_squares(10);
        let expected = 385;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example_square_of_sums() {
        let actual = square_of_sums(10);
        let expected = 3025;

        assert_eq!(actual, expected);
    }
}
