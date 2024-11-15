use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct NumVec(pub String);

impl NumVec {
    fn is_each_digit_valid(&self) -> bool {
        self.0.chars().all(|c| c.is_ascii_digit())
    }

    pub fn new(num: String) -> Self {
        if NumVec(num.clone()).is_each_digit_valid() {
            NumVec(num)
        } else {
            panic!("Invalid number")
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn n_from_start(&self, n: usize) -> u32 {
        let digit = self.0.chars().nth(n).unwrap();
        digit.to_digit(10).unwrap()
    }

    pub fn n_from_end(&self, n: usize) -> u32 {
        let len = self.len();
        let index = len - n - 1;
        let digit = self.0.chars().nth(index).unwrap();
        digit.to_digit(10).unwrap()
    }

    pub fn to_vec_with_size(&self, size: usize) -> Vec<u32> {
        let mut vec = vec![0; size];
        for i in 0..self.len() {
            vec[size - self.len() + i] = self.n_from_start(i);
        }
        vec
    }

    pub fn add_digits(nums: Vec<Self>) -> Self {
        let len = nums.iter().map(|n| n.len()).max().unwrap();

        let vecs = nums
            .iter()
            .map(|n| n.to_vec_with_size(len))
            .collect::<Vec<Vec<u32>>>();

        let mut sum_vector = vec![0; len];
        vecs.iter().for_each(|vec| {
            sum_vector = Self::get_element_wise_sum_vector(len, sum_vector.clone(), vec.clone());
        });

        let result = Self::convert_sum_vector_to_sum_string(len, &mut sum_vector);

        NumVec::from_str(&result).expect("Failed to create NumVec from str")
    }

    fn get_element_wise_sum_vector(len: usize, vec1: Vec<u32>, vec2: Vec<u32>) -> Vec<u32> {
        let mut sum_vector = vec![0; len];
        (0..len).rev().for_each(|i| {
            sum_vector[i] += vec1[i] + vec2[i];
        });
        sum_vector
    }

    fn convert_sum_vector_to_sum_string(len: usize, sum_vector: &mut Vec<u32>) -> String {
        let mut sum_overflow = 0;
        let mut sum_overflow_str: String = "".to_string();

        (0..len).rev().for_each(|i| {
            while sum_vector[i] >= 10 {
                sum_vector[i] -= 10;
                if i == 0 {
                    sum_overflow += 1;
                } else {
                    sum_vector[i - 1] += 1;
                }
            }
        });

        if sum_overflow != 0 {
            sum_overflow_str = sum_overflow.to_string();
        }

        let result = sum_overflow_str
            + &sum_vector
                .iter()
                .map(|&d| d.to_string())
                .collect::<String>();
        result
    }
}

impl Iterator for NumVec {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            let next = self.0.chars().next();
            self.0 = self.0.chars().skip(1).collect();
            next
        }
    }
}

impl FromStr for NumVec {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NumVec::new(s.to_string()))
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    fn nums() -> (String, String) {
        (
            "37107287533902102798797998220837590246510135740250".to_string(),
            "46376937677490009712648124896970078050417018260538".to_string(),
        )
    }

    #[test]
    fn can_get_digit_n_from_end() {
        let (n1, n2) = nums();
        let n1 = NumVec::new(n1);
        let n2 = NumVec::new(n2);

        assert_eq!(0, n1.n_from_end(0));
        assert_eq!(8, n2.n_from_end(0));

        assert_eq!(5, n1.n_from_end(1));
        assert_eq!(3, n2.n_from_end(1));
    }

    #[test]
    fn can_get_digit_n_from_start() {
        let (n1, n2) = nums();
        let n1 = NumVec::new(n1);
        let n2 = NumVec::new(n2);

        assert_eq!(3, n1.n_from_start(0));
        assert_eq!(4, n2.n_from_start(0));

        assert_eq!(7, n1.n_from_start(1));
        assert_eq!(6, n2.n_from_start(1));
    }

    #[test]
    fn can_represent_50_digit_number_as_vector_of_digits() {
        let num = "37107287533902102798797998220837590246510135740250";
        let num_vec = NumVec::new(num.to_string());

        assert_eq!(50, num_vec.0.len());
    }

    #[test]
    fn can_get_len_of_number_represented_as_vector_of_digits() {
        let num = "37107287533902102798797998220837590246510135740250";
        let num_vec = NumVec::new(num.to_string());

        assert_eq!(50, num_vec.len());
    }

    #[test]
    fn can_test_whether_number_is_empty() {
        let num = "37107287533902102798797998220837590246510135740250";
        let num_vec = NumVec::new(num.to_string());

        assert!(!num_vec.is_empty());
    }

    #[test]
    #[should_panic]
    fn panics_when_number_contains_non_digit_characters() {
        let num = "3710728753390210279879799822083759024651013574025a";
        NumVec::new(num.to_string());
    }

    #[test]
    fn can_iterate_over_digits_in_number() {
        let num = "37107287533902102798797998220837590246510135740250";
        let num_vec = NumVec::new(num.to_string());

        let mut iter = num_vec.into_iter();

        assert_eq!(Some('3'), iter.next());
        assert_eq!(Some('7'), iter.next());
        assert_eq!(Some('1'), iter.next());
        assert_eq!(Some('0'), iter.next());
        assert_eq!(Some('7'), iter.next());
        assert_eq!(Some('2'), iter.next());
        assert_eq!(Some('8'), iter.next());
        assert_eq!(Some('7'), iter.next());
        assert_eq!(Some('5'), iter.next());
    }

    #[test]
    fn can_convert_number_to_vector_of_digits() {
        let num = "37107287533902102798797998220837590246510135740250";
        let num_vec = NumVec::new(num.to_string());

        let vec = num_vec.to_vec_with_size(50);
        let expected = vec![
            3, 7, 1, 0, 7, 2, 8, 7, 5, 3, 3, 9, 0, 2, 1, 0, 2, 7, 9, 8, 7, 9, 7, 9, 9, 8, 2, 2, 0,
            8, 3, 7, 5, 9, 0, 2, 4, 6, 5, 1, 0, 1, 3, 5, 7, 4, 0, 2, 5, 0,
        ];

        assert_eq!(vec, expected);
    }

    #[test]
    fn converting_number_to_vector_pads_with_zeros() {
        let num = "123";
        let num_vec = NumVec::new(num.to_string());

        let vec = num_vec.to_vec_with_size(5);
        let expected = vec![0, 0, 1, 2, 3];

        assert_eq!(vec, expected);
    }

    #[test]
    fn can_create_numvec_from_str() {
        let num = "37107287533902102798797998220837590246510135740250";
        let num_vec = NumVec::new(num.to_string());

        let num_vec_from_str = NumVec::from_str(num);

        assert_eq!(
            num_vec,
            num_vec_from_str.expect("Failed to create NumVec from str")
        );
    }

    #[test]
    fn can_convert_from_vector_of_digits_to_number() {
        let num = "37107287533902102798797998220837590246510135740250".to_string();
        let num_vec = NumVec::new(num);

        let vec = num_vec.to_vec_with_size(50);
        let re_num = NumVec::from_str(&vec.iter().map(|&d| d.to_string()).collect::<String>())
            .expect("Failed to create NumVec from str");

        assert_eq!(num_vec, re_num);
    }

    #[test]
    fn can_add_two_numbers_as_vectors() {
        let n1 = NumVec::new("123".to_owned());
        let n2 = NumVec::new("456".to_owned());

        let result = NumVec::add_digits(vec![n1, n2]);
        let expected = NumVec::new("579".to_owned());

        assert_eq!(result, expected);
    }

    #[test]
    fn can_add_two_numbers_with_carry() {
        let n1 = NumVec::new("123".to_owned());
        let n2 = NumVec::new("457".to_owned());

        let result = NumVec::add_digits(vec![n1, n2]);
        let expected = NumVec::new("580".to_owned());

        assert_eq!(result, expected);
    }

    #[test]
    fn can_add_two_numbers_with_different_lengths() {
        let n1 = NumVec::new("123".to_owned());
        let n2 = NumVec::new("4567".to_owned());

        let result = NumVec::add_digits(vec![n1, n2]);
        let expected = NumVec::new("4690".to_owned());

        assert_eq!(result, expected);
    }

    #[test]
    fn can_add_two_numbers_with_different_lengths_and_carry() {
        let n1 = NumVec::new("222".to_owned());
        let n2 = NumVec::new("9999".to_owned());

        let result = NumVec::add_digits(vec![n1, n2]);
        let expected = NumVec::new("10221".to_owned());

        assert_eq!(result, expected);
    }

    #[test]
    fn can_add_three_numbers() {
        let n1 = NumVec::new("1111".to_owned());
        let n2 = NumVec::new("2222".to_owned());
        let n3 = NumVec::new("3333".to_owned());

        let result = NumVec::add_digits(vec![n1, n2, n3]);
        let expected = NumVec::new("6666".to_owned());

        assert_eq!(result, expected);
    }

    #[test]
    fn can_add_three_numbers_with_carry() {
        let n1 = NumVec::new("999".to_owned());
        let n2 = NumVec::new("999".to_owned());
        let n3 = NumVec::new("999".to_owned());

        let result = NumVec::add_digits(vec![n1, n2, n3]);
        let expected = NumVec::new("2997".to_owned());

        assert_eq!(result, expected);
    }
}
