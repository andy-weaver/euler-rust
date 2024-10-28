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

    pub fn get_primes_below_n(&mut self, n: u64) -> Vec<u64> {
        self.parallel_primes_up_to(n)
    }

    pub fn sum_primes_below_n(&mut self, n: u64) -> u64 {
        let primes = self.get_primes_below_n(n);
        primes.iter().sum()
    }

    pub fn get_next_prime(&mut self) -> u64 {
        match self.next() {
            Some(p) => p,
            None => self.next().expect(
                "There is an infinite number of primes so this iterator should never stop...",
            ),
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

    pub fn is_prime_with_small_primes(candidate_prime: u64, small_primes: &[u64]) -> bool {
        small_primes
            .iter()
            .all(|&prime| candidate_prime % prime != 0)
    }

    fn get_primes_below_n_with_sieve(limit: u64) -> Vec<u64> {
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

    fn parallel_primes_up_to(&mut self, n: u64) -> Vec<u64> {
        let parallel_processing_threshold = 50;

        if n <= 2 {
            return vec![2];
        } else if n < parallel_processing_threshold {
            return self.get_primes_below_n_in_series(n);
        }

        let small_primes = Self::generate_small_primes(n);
        let primes_in_parallel = Self::mark_larger_primes_parallel(n, small_primes.as_slice());

        primes_in_parallel
            .iter()
            .filter(|k| k < &&n)
            .copied()
            .collect()
    }

    fn get_small_prime_limit(n: u64) -> u64 {
        (n as f64).sqrt().ceil() as u64
    }

    fn generate_small_primes(n: u64) -> Vec<u64> {
        let limit = Self::get_small_prime_limit(n);
        Self::get_primes_below_n_with_sieve(limit)
    }

    fn mark_larger_primes_parallel(n: u64, small_primes: &[u64]) -> Vec<u64> {
        let primes_in_parallel = Mutex::new(small_primes.to_vec());
        let small_prime_limit = Self::get_small_prime_limit(n);

        (small_prime_limit + 1..n)
            .into_par_iter()
            .filter(|&candidate| Self::is_prime_with_small_primes(candidate, small_primes))
            .for_each(|candidate| {
                let mut primes = primes_in_parallel.lock().unwrap();
                primes.push(candidate);
            });

        let mut result = primes_in_parallel.into_inner().unwrap();
        result.sort_unstable();
        result
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
    fn can_get_current_max_prime_before_and_after_adding_a_new_one() {
        let mut p = PrimeGenerator::new();
        assert_eq!(p.current_max().unwrap(), 2);

        p.add_prime(3);
        assert_eq!(p.current_max().unwrap(), 3);
    }

    fn get_nth_prime(n: u64) -> Option<u64> {
        let mut p = PrimeGenerator::new();
        p.nth_prime(n)
    }

    #[test]
    fn test_nth_prime() {
        (1..=50).for_each(|n| {
            let mut p = PrimeGenerator::new();
            assert_eq!(p.nth_prime(n), get_nth_prime(n));
        });
    }

    fn test_series_equivalent_to_parallel_for_n(n: u64) {
        let mut p = PrimeGenerator::new();
        let series = p.get_primes_below_n_in_series(n);
        let parallel = p.parallel_primes_up_to(n);

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
    fn getting_primes_in_series_is_equivalent_to_in_parallel_for_1000() {
        test_series_equivalent_to_parallel_for_n(1000);
    }

    #[ignore]
    #[test]
    fn getting_primes_in_series_is_equivalent_to_in_parallel_for_10_000() {
        test_series_equivalent_to_parallel_for_n(10000);
    }

    #[test]
    fn test_sum_primes_lt_10() {
        let mut p = PrimeGenerator::new();
        let expected = 17;
        let actual = p.sum_primes_below_n(10);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sum_primes_lt_11_does_not_include_11() {
        let mut p = PrimeGenerator::new();
        let expected = 17;
        let actual = p.sum_primes_below_n(11);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sum_primes_lt_12_does_include_11() {
        let mut p = PrimeGenerator::new();
        let expected = 17 + 11;
        let actual = p.sum_primes_below_n(12);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sieve_up_to_10() {
        let primes = PrimeGenerator::get_primes_below_n_with_sieve(10);
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_sieve_up_to_30() {
        let primes = PrimeGenerator::get_primes_below_n_with_sieve(30);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_is_prime_with_small_primes_only() {
        let small_primes = vec![2, 3, 5, 7];

        assert!(PrimeGenerator::is_prime_with_small_primes(
            11,
            &small_primes
        ));
        assert!(!PrimeGenerator::is_prime_with_small_primes(
            12,
            &small_primes
        ));
    }

    #[test]
    fn test_get_primes_below_10() {
        let mut generator = PrimeGenerator::new();
        let primes = generator.get_primes_below_n(10);
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_get_primes_below_30() {
        let mut generator = PrimeGenerator::new();
        let primes = generator.get_primes_below_n(30);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    fn primes_below_1000() -> Vec<u64> {
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997,
        ]
    }

    #[test]
    fn test_get_primes_below_1000() {
        let mut p = PrimeGenerator::new();
        let actual = p.get_primes_below_n(1000);
        let expected = primes_below_1000();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_empty() {
        let mut p = PrimeGenerator::new();
        assert!(!p.is_empty());

        p.primes.pop();
        assert!(p.is_empty());
    }

    #[test]
    fn test_0th_prime() {
        let mut p = PrimeGenerator::new();
        assert!(p.nth_prime(0).is_none());
    }

    #[test]
    fn test_primes_up_to_0() {
        let mut p = PrimeGenerator::new();
        let actual = p.parallel_primes_up_to(0);
        let expected = vec![2];

        assert_eq!(actual, expected);
    }
    #[test]
    fn test_primes_up_to_1() {
        let mut p = PrimeGenerator::new();
        let actual = p.parallel_primes_up_to(1);
        let expected = vec![2];

        assert_eq!(actual, expected);
    }
}
