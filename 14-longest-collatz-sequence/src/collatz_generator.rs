pub fn get_collatz_sequence_len(n: u64) -> usize {
    let mut collatz = CollatzGenerator::new(n);
    collatz.len()
}

#[derive(Debug, PartialEq)]
pub struct CollatzGenerator {
    starting_number: u64,
    current_number: u64,
}

impl CollatzGenerator {
    pub fn new(starting_number: u64) -> Self {
        CollatzGenerator {
            starting_number,
            current_number: starting_number,
        }
    }

    pub fn is_even(&self) -> bool {
        self.current_number % 2 == 0
    }

    pub fn is_odd(&self) -> bool {
        !self.is_even()
    }

    pub fn calculate_next_term_when_even(&mut self) -> Option<u64> {
        self.current_number /= 2;
        Some(self.current_number)
    }

    pub fn calculate_next_term_when_odd(&mut self) -> Option<u64> {
        self.current_number *= 3;
        self.current_number += 1;
        Some(self.current_number)
    }

    pub fn calculate_next_term(&mut self) -> Option<u64> {
        if self.current_number == 1 {
            None
        } else if self.is_even() {
            self.calculate_next_term_when_even()
        } else {
            self.calculate_next_term_when_odd()
        }
    }

    pub fn get_collatz_sequence(&mut self) -> Vec<u64> {
        let mut sequence = vec![self.starting_number];
        while let Some(next_term) = self.calculate_next_term() {
            sequence.push(next_term);
        }
        sequence
    }

    pub fn len(&mut self) -> usize {
        self.get_collatz_sequence().len()
    }

    #[must_use]
    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }
}

impl Iterator for CollatzGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.calculate_next_term()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let collatz = CollatzGenerator::new(10);
        assert_eq!(collatz.starting_number, 10);

        let collatz2 = CollatzGenerator {
            starting_number: 10,
            current_number: 10,
        };

        assert_eq!(collatz, collatz2);
    }

    #[test]
    fn can_test_is_even() {
        let collatz = CollatzGenerator::new(10);
        assert!(collatz.is_even());

        let collatz = CollatzGenerator::new(11);
        assert!(!collatz.is_even());
    }

    #[test]
    fn can_test_is_odd() {
        let collatz = CollatzGenerator::new(10);
        assert!(!collatz.is_odd());

        let collatz = CollatzGenerator::new(11);
        assert!(collatz.is_odd());
    }

    #[test]
    fn can_calculate_next_term_when_even() {
        let mut collatz = CollatzGenerator::new(10);
        assert_eq!(collatz.current_number, 10);

        assert_eq!(collatz.calculate_next_term_when_even(), Some(5));
    }

    #[test]
    fn can_calculate_next_term_when_odd() {
        let mut collatz = CollatzGenerator::new(11);
        assert_eq!(collatz.current_number, 11);

        assert_eq!(collatz.calculate_next_term_when_odd(), Some(11 * 3 + 1));
    }

    #[test]
    fn can_calculate_next_term() {
        let mut collatz = CollatzGenerator::new(10);
        assert_eq!(collatz.current_number, 10);

        assert_eq!(collatz.calculate_next_term(), Some(5));
        assert_eq!(collatz.calculate_next_term(), Some(16));
        assert_eq!(collatz.calculate_next_term(), Some(8));
        assert_eq!(collatz.calculate_next_term(), Some(4));
        assert_eq!(collatz.calculate_next_term(), Some(2));
        assert_eq!(collatz.calculate_next_term(), Some(1));
    }

    #[test]
    fn calculate_next_term_returns_none_when_current_value_is_1() {
        let mut collatz = CollatzGenerator::new(2);
        assert_eq!(collatz.current_number, 2);
        assert_eq!(collatz.calculate_next_term(), Some(1));
        assert_eq!(collatz.calculate_next_term(), None);
    }

    #[test]
    fn can_get_collatz_sequence() {
        let mut collatz = CollatzGenerator::new(10);
        assert_eq!(collatz.get_collatz_sequence(), vec![10, 5, 16, 8, 4, 2, 1]);

        let mut collatz = CollatzGenerator::new(13);
        assert_eq!(
            collatz.get_collatz_sequence(),
            vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]
        );
    }

    #[test]
    fn can_iterate() {
        let collatz = CollatzGenerator::new(10);
        assert_eq!(collatz.collect::<Vec<u64>>(), vec![5, 16, 8, 4, 2, 1]);

        let collatz = CollatzGenerator::new(13);
        assert_eq!(
            collatz.collect::<Vec<u64>>(),
            vec![40, 20, 10, 5, 16, 8, 4, 2, 1]
        );
    }

    #[test]
    fn can_get_len_of_collatz_sequence() {
        let mut collatz = CollatzGenerator::new(10);
        assert_eq!(collatz.len(), 7);

        let mut collatz = CollatzGenerator::new(13);
        assert_eq!(collatz.len(), 10);
    }
}
