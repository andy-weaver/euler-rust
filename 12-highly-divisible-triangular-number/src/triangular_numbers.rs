pub fn get_first_with_n_divisors(n: u32) -> u32 {
    let i = (1..).find(|&i| count_divisors_of_number(get_triangular_number(i)) > n).unwrap();
    get_triangular_number(i)
}

pub fn get_triangular_number(n: u32) -> u32 {
    n * (n + 1) / 2
}

pub fn count_divisors_of_number(n: u32) -> u32 {
    let mut divisors = vec![];
    let max_search_term = (n as f64).sqrt() as u32;

    (1..=max_search_term).filter(|i| n % i == 0).for_each(|i| {
        divisors.push(i);
        if i != n / i {
            divisors.push(n / i);
        }
    });

    divisors.len() as u32
}

pub fn get_divisors_of_number(n: u32) -> Vec<u32> {
    let mut divisors = vec![];
    let max_search_term = (n as f64).sqrt() as u32;

    (1..=max_search_term).filter(|i| n % i == 0).for_each(|i| {
        divisors.push(i);
        if i != n / i {
            divisors.push(n / i);
        }
    });

    divisors.sort();
    divisors
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn can_get_triangular_number() {
        assert_eq!(get_triangular_number(1), 1);
        assert_eq!(get_triangular_number(2), 3);
        assert_eq!(get_triangular_number(3), 6);
        assert_eq!(get_triangular_number(4), 10);
        assert_eq!(get_triangular_number(5), 15);
    }

    #[test]
    fn can_get_divisors_of_number() {
        assert_eq!(get_divisors_of_number(1), vec![1]);
        assert_eq!(get_divisors_of_number(2), vec![1, 2]);
        assert_eq!(get_divisors_of_number(3), vec![1, 3]);
        assert_eq!(get_divisors_of_number(4), vec![1, 2, 4]);
        assert_eq!(get_divisors_of_number(10), vec![1, 2, 5, 10]);
    }

    #[test]
    fn can_get_number_of_divisors_of_number() {
        assert_eq!(count_divisors_of_number(1), 1);
        assert_eq!(count_divisors_of_number(2), 2);
        assert_eq!(count_divisors_of_number(3), 2);
        assert_eq!(count_divisors_of_number(4), 3);
        assert_eq!(count_divisors_of_number(10), 4);
    }

    #[test]
    fn can_get_first_triangular_number_with_3_divisors() {
        assert_eq!(get_first_with_n_divisors(3), 6);
    }

    #[test]
    fn can_get_first_triangular_number_with_5_divisors() {
        assert_eq!(get_first_with_n_divisors(5), 28);
    }
}
