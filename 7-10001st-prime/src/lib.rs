pub mod prime_getter;

#[derive(Debug, Clone, PartialEq)]
pub struct Primes {
    n_primes: u64,
    primes: Vec<u64>,
}

impl Primes {
    pub fn new() -> Self {
        Primes {
            n_primes: 0,
            primes: vec![],
        }
    }

    fn add_prime(&mut self, x: &u64) {
        self.primes.push(*x);
        self.n_primes += 1;
    }

    pub fn fill_primes_until(&mut self, n: u64) {
        let start = self.current_max_prime();
        (start..=n).for_each(|x| {
            if self.is_prime(&x) {
                self.add_prime(&x);
            }
        })
    }

    pub fn contains(&self, n: &u64) -> bool {
        self.primes.contains(n)
    }

    pub fn is_prime(&self, n: &u64) -> bool {
        !self.primes.iter().filter(|x| *x != n).any(|x| n % x == 0)
    }

    pub fn current_max_prime(&self) -> u64 {
        if self.n_primes > 0 {
            self.primes[self.n_primes as usize - 1usize]
        } else {
            2
        }
    }

    pub fn nth(&self, n: usize) -> u64 {
        *self.primes.iter().nth(n + 1).unwrap() as u64
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        Some(*self.primes.iter().next().unwrap())
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_construct_new_struct() {
        let p1 = Primes {
            n_primes: 0,
            primes: vec![],
        };
        let p2 = Primes::new();

        assert_eq!(p1, p2);
    }

    #[test]
    fn test_add_prime() {
        let mut p = Primes::new();
        p.add_prime(&2);

        assert_eq!(p.n_primes, 1);
        assert_eq!(p.primes, vec![2]);
    }

    #[test]
    fn test_fill_primes_until_3() {
        let mut p = Primes::new();
        p.fill_primes_until(3);

        assert!(p.contains(&2));
        assert!(p.contains(&3));
        assert!(!p.contains(&4));
    }

    #[test]
    fn test_fill_primes_until_5() {
        let mut p = Primes::new();
        p.fill_primes_until(5);

        assert!(p.contains(&2));
        assert!(p.contains(&3));
        assert!(!p.contains(&4));
        assert!(p.contains(&5));
        assert!(!p.contains(&6));
    }

    #[test]
    fn test_n_primes_below_m() {
        let mut p = Primes::new();
        p.fill_primes_until(10);
        assert_eq!(p.n_primes, 4);

        let mut p = Primes::new();
        p.fill_primes_until(20);
        assert_eq!(p.n_primes, 8);

        let mut p = Primes::new();
        p.fill_primes_until(30);
        assert_eq!(p.n_primes, 10);
    }

    #[test]
    fn test_is_prime_for_numbers_1_thru_10() {
        let mut p = Primes::new();
        p.fill_primes_until(10);

        println!("{:?}", p.primes);

        assert!(p.is_prime(&2));
        assert!(p.is_prime(&3));
        assert!(p.is_prime(&5));
        assert!(p.is_prime(&7));
        assert!(!p.is_prime(&4));
        assert!(!p.is_prime(&6));
        assert!(!p.is_prime(&8));
        assert!(!p.is_prime(&9));
        assert!(!p.is_prime(&10));
    }

    #[test]
    fn test_current_max_prime() {
        let mut p = Primes::new();
        assert_eq!(p.current_max_prime(), 2);
        p.fill_primes_until(10);
        assert_eq!(p.current_max_prime(), 7);
        p.fill_primes_until(20);
        assert_eq!(p.current_max_prime(), 19);
    }
}
