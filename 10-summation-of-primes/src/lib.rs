use rayon::prelude::*;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Clone)]
pub struct PrimeGenerator {
    pub primes: Vec<u64>,
    n: u64,
}

impl PrimeGenerator {
    pub fn new() -> Self {
        PrimeGenerator {
            primes: vec![2],
            n: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.primes.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn current_max(&self) -> Option<u64> {
        self.primes.last().cloned()
    }

    pub fn add_prime(&mut self, n: u64) {
        self.primes.push(n);
        self.n += 1;
    }

    pub fn is_prime(&self, candidate_prime: u64) -> bool {
        self.primes.iter().all(|prime| candidate_prime % prime != 0)
    }

    pub fn nth_prime(&mut self, n: u64) -> Option<u64> {
        if n == 0 {
            None
        } else if n == 1 {
            return Some(2);
        } else {
            return self.nth((n - 2).try_into().unwrap_or(2));
        }
    }

    pub fn get_primes_below_n_with_sieve(limit: u64) -> Vec<u64> {
        let mut sieve = vec![true; (limit + 1) as usize];
        sieve[0] = false;
        sieve[1] = false;

        for i in 2..=((limit as f64).sqrt() as usize) {
            if sieve[i] {
                (i * i..=limit as usize)
                    .step_by(i)
                    .for_each(|multiple| sieve[multiple] = false);
            }
        }

        sieve
            .iter()
            .enumerate()
            .filter_map(|(i, &is_prime)| if is_prime { Some(i as u64) } else { None })
            .collect::<Vec<u64>>()
    }

    pub fn is_prime_for_small_primes(candidate_prime: u64, small_primes: &[u64]) -> bool {
        small_primes.iter().all(|&prime| candidate_prime % prime != 0)
    }

    pub fn get_primes_below_n(&mut self, n: u64) -> Vec<u64> {
            self.get_primes_below_n_in_parallel(n)
    }

    pub fn sum_primes_below_n(&mut self, n: u64) -> u64 {
        let primes = self.get_primes_below_n(n);
        primes.iter().sum()
    }

    fn get_primes_below_n_in_series(&mut self, n: u64) -> Vec<u64> {
        (0..n).for_each(|_| {
            self.next();
        });

        let primes_lt_n = self
            .primes
            .iter()
            .filter(|k| k < &&n)
            .map(|k| k.to_owned())
            .collect::<Vec<u64>>();

        primes_lt_n
    }

    fn get_primes_below_n_in_parallel(&mut self, n: u64) -> Vec<u64> {
        let parallel_processing_threshold = 50;

        if n <= 2 {
            return vec![2];
        } else if n < parallel_processing_threshold {
            return self.get_primes_below_n_in_series(n);
        }

        let small_primes = Self::_generate_small_primes_up_to_sqrt_n(n);

        let primes_in_parallel = Mutex::new(vec![]);

        (parallel_processing_threshold..n)
            .into_par_iter()
            .filter(|k| k % 2 != 0)
            .for_each(|candidate| {
                if self.is_prime(candidate) {
                    let mut primes = primes_in_parallel.lock().unwrap();
                    primes.push(candidate);
                }
            });

        let mut result = self.primes.clone();
        result.append(&mut primes_in_parallel.into_inner().unwrap());
        result.sort_unstable();

        result.iter().filter(|k| k < &&n).copied().collect()
    }

    fn _generate_small_prime_limit(n: u64) -> u64 {
        (n as f64).sqrt().ceil() as u64
    }

    fn _generate_small_primes_up_to_sqrt_n(n: u64) -> Vec<u64> {
        let limit = Self::_generate_small_prime_limit(n);
        Self::get_primes_below_n_with_sieve(limit)
    }

    fn _use_generated_small_primes_to_mark_larger_primes_in_parallel(n: u64, small_primes: Vec<u64>) -> Vec<u64> { 
        let primes_in_parallel = Mutex::new(small_primes.clone());
        let small_prime_limit = Self::_generate_small_prime_limit(n);

        (small_prime_limit+1..n)
            .into_par_iter()
            .filter(|&candidate| Self::is_prime_for_small_primes(candidate, small_primes.as_slice()))
            .for_each(|&candidate| {
                let mut primes = primes_in_parallel.lock().unwrap();
                primes.push(candidate);
            });

    }
}

impl Default for PrimeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for PrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.current_max().unwrap()..).find(|x| self.is_prime(*x));
        match result {
            Some(p) => {
                self.add_prime(p);
                Some(p)
            }
            None => {
                println!("I don't think this is possible unless I screwed up.");
                None
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn can_make_object() {
        let actual = PrimeGenerator::new();
        let expected = PrimeGenerator {
            primes: vec![2],
            n: 1,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_get_len_of_primes() {
        let p = PrimeGenerator::new();

        assert_eq!(p.len(), 1);
    }

    #[test]
    fn can_push_new_prime() {
        let mut p = PrimeGenerator::new();
        p.add_prime(3);

        assert_eq!(p.len(), 2);
    }

    #[test]
    fn can_test_next_number_to_see_if_it_is_prime() {
        let mut p = PrimeGenerator::new();

        assert!(p.is_prime(3));

        p.add_prime(3);

        assert!(!p.is_prime(4));
    }

    #[test]
    fn can_get_current_max_prime() {
        let mut p = PrimeGenerator::new();

        assert_eq!(p.current_max().unwrap(), 2);

        p.add_prime(3);

        assert_eq!(p.current_max().unwrap(), 3);
    }

    #[test]
    fn can_get_next_prime() {
        let mut p = PrimeGenerator::new();

        assert_eq!(p.next().unwrap(), 3);
        assert_eq!(p.next().unwrap(), 5);
        assert_eq!(p.next().unwrap(), 7);
        assert_eq!(p.next().unwrap(), 11);
        assert_eq!(p.next().unwrap(), 13);
        assert_eq!(p.next().unwrap(), 17);
        assert_eq!(p.next().unwrap(), 19);
    }

    #[test]
    fn test_6th_prime() {
        let mut p = PrimeGenerator::new();
        assert_eq!(p.next().unwrap(), 3);
    }

    #[test]
    fn test_nth_prime() {
        let mut p = PrimeGenerator::new();
        assert_eq!(p.nth_prime(1), Some(2));
        let mut p = PrimeGenerator::new();
        assert_eq!(p.nth_prime(2), Some(3));
        let mut p = PrimeGenerator::new();
        assert_eq!(p.nth_prime(3), Some(5));
        let mut p = PrimeGenerator::new();
        assert_eq!(p.nth_prime(4), Some(7));
        let mut p = PrimeGenerator::new();
        assert_eq!(p.nth_prime(5), Some(11));
        let mut p = PrimeGenerator::new();
        assert_eq!(p.nth_prime(6), Some(13));
    }

    #[test]
    fn can_get_primes_below_10_in_series() {
        let mut p = PrimeGenerator::new();
        let primes_lt_10 = p.get_primes_below_n(10, false);
        assert_eq!(primes_lt_10, vec![2, 3, 5, 7]);
    }

    #[test]
    fn can_get_primes_below_10_in_parallel() {
        let mut p = PrimeGenerator::new();
        let primes_lt_10 = p.get_primes_below_n(10, true);
        assert_eq!(primes_lt_10, vec![2, 3, 5, 7]);
    }

    fn test_series_equivalent_to_parallel_for_n(n: u64) {
        let mut p = PrimeGenerator::new();
        let series = p.get_primes_below_n_in_series(n);
        let parallel = p.get_primes_below_n_in_parallel(n);

        assert_eq!(series, parallel);
    }

    #[test]
    fn getting_primes_in_series_is_equivalent_to_in_parallel_for_50() {
        test_series_equivalent_to_parallel_for_n(50);
    }
    #[test]
    fn getting_primes_in_series_is_equivalent_to_in_parallel_for_100() {
        test_series_equivalent_to_parallel_for_n(100);
    }

    #[test]
    fn getting_primes_in_series_is_equivalent_to_in_parallel_for_20_000() {
        test_series_equivalent_to_parallel_for_n(20000);
    }

    #[test]
    fn test_sum_primes_lt_10() {
        let mut p = PrimeGenerator::new();
        let expected = 17;
        let actual = p.sum_primes_below_n(10, true);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sum_primes_lt_11_does_not_include_11() {
        let mut p = PrimeGenerator::new();
        let expected = 17;
        let actual = p.sum_primes_below_n(11, true);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sum_primes_lt_12_does_include_11() {
        let mut p = PrimeGenerator::new();
        let expected = 17 + 11;
        let actual = p.sum_primes_below_n(12, true);

        assert_eq!(actual, expected);
    }
}
