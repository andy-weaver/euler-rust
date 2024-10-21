pub struct MultiplesOf {
    pub number: i32,
    pub multiples: Vec<i32>,
}

impl MultiplesOf {
    pub fn new(number: i32) -> Self {
        MultiplesOf {
            number,
            multiples: vec![],
        }
    }

    pub fn is_multiple_of(&self, x: &i32) -> bool {
        x % self.number == 0
    }

    pub fn add(&mut self, x: &i32) {
        self.multiples.push(*x);
    }

    pub fn get_multiples_below(&mut self, limit: i32) {
        let mults: Vec<i32> = (1..limit).filter(|x| self.is_multiple_of(x)).collect();
        mults.iter().for_each(|x| self.add(x));
    }

    pub fn sum_multiples(&self) -> i32 {
        self.multiples.iter().sum()
    }
}

impl Iterator for MultiplesOf {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.multiples.pop()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_is_multiple_of_3() {
        let m = MultiplesOf::new(3);
        assert!(m.is_multiple_of(&12));
    }

    #[test]
    fn test_is_not_multiple_of_3() {
        let m = MultiplesOf::new(3);
        assert!(!m.is_multiple_of(&13));
    }

    #[test]
    fn test_is_multiple_of_5() {
        let m = MultiplesOf::new(5);
        assert!(m.is_multiple_of(&25));
    }

    #[test]
    fn test_is_not_multiple_of_5() {
        let m = MultiplesOf::new(5);
        assert!(!m.is_multiple_of(&26));
    }

    #[test]
    fn test_add_1_number() {
        let mut m = MultiplesOf::new(3);
        m.add(&3);

        assert_eq!(m.multiples.len(), 1);
        assert_eq!(m.multiples.pop().unwrap(), 3);
    }

    #[test]
    fn test_add_3_numbers() {
        let mut m = MultiplesOf::new(5);
        m.add(&5);
        m.add(&10);
        m.add(&15);

        assert_eq!(m.multiples.len(), 3);
        assert_eq!(m.multiples.pop().unwrap(), 15);

        assert_eq!(m.multiples.pop().unwrap(), 10);
        assert_eq!(m.multiples.pop().unwrap(), 5);
        assert_eq!(m.multiples.pop(), None);
    }

    #[test]
    fn test_get_multiples_below_limit() {
        let mut m = MultiplesOf::new(3);
        m.get_multiples_below(10);

        assert_eq!(m.multiples.len(), 3);
        assert!(m.multiples.contains(&3));
        assert!(m.multiples.contains(&6));
        assert!(m.multiples.contains(&9));
    }

    #[test]
    fn test_get_multiples_strictly_below_limit() {
        let mut m = MultiplesOf::new(5);
        m.get_multiples_below(10);

        assert_eq!(m.multiples.len(), 1);
        assert!(m.multiples.contains(&5));
        assert!(!m.multiples.contains(&10));
    }

    #[test]
    fn test_sum_multiples_below_limit() {
        let mut m = MultiplesOf::new(3);
        m.get_multiples_below(10);

        assert_eq!(m.sum_multiples(), 3 + 6 + 9);
    }

    #[test]
    fn test_from_problem_statement() {
        let mut m3 = MultiplesOf::new(3);
        let mut m5 = MultiplesOf::new(5);
        let mut m15 = MultiplesOf::new(15);

        m3.get_multiples_below(10);
        m5.get_multiples_below(10);
        m15.get_multiples_below(10);

        let sum_mults = m3.sum_multiples() + m5.sum_multiples() - m15.sum_multiples();

        assert_eq!(sum_mults, 23);
    }
}
