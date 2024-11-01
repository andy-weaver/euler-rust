use crate::prime_generator::PrimeGenerator;

#[derive(Debug, PartialEq)]
pub struct IntegerFactorizer {
    pub k: usize,
}

impl IntegerFactorizer {
    pub fn new(k: usize) -> Self {
        IntegerFactorizer { k }
    }

    pub fn sqrt_ceiling(&self) -> usize {
        (self.k as f64).sqrt().ceil() as usize
    }

    pub fn is_prime(&self) -> bool {
        let mut p = PrimeGenerator::new();
        p.get_primes_below_n(self.k + 1);
        p.is_prime(self.k)
    }

    pub fn prime_factors(&self) -> Vec<usize> {
        let sqrt = self.sqrt_ceiling();
        let mut p = PrimeGenerator::new();
        p.get_primes_below_n(sqrt);
        p.primes
            .iter()
            .map(|m| *m)
            .filter(|&prime| self.k % prime == 0)
            .collect::<Vec<usize>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_integer_factorizer() {
        let f1 = IntegerFactorizer { k: 10 };
        let f2 = IntegerFactorizer::new(10);

        assert_eq!(f1, f2);
    }

    #[test]
    fn can_take_square_root_of_integer() {
        let f = IntegerFactorizer::new(9);
        assert_eq!(f.sqrt_ceiling(), 3);
    }

    #[test]
    fn can_get_prime_factors_of_2() {
        let f = IntegerFactorizer::new(2);
        assert_eq!(f.prime_factors(), vec![2]);
    }

    #[test]
    fn can_test_whether_integer_is_prime() {
        let f = IntegerFactorizer::new(3);
        assert!(f.is_prime());
    }

    fn get_prime_factors_of_integer(k: usize) -> Vec<usize> {
        let f = IntegerFactorizer::new(k);
        f.prime_factors()
    }

    #[test]
    fn can_get_prime_factors_of_integers_below_20() {
        assert_eq!(get_prime_factors_of_integer(2), vec![2]);
        // assert_eq!(get_prime_factors_of_integer(3), vec![3]);
    }
}
