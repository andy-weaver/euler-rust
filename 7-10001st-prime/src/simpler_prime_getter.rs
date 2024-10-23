#[derive(Debug, PartialEq)]
pub struct SimplerPrimeGetter {
    pub primes: Vec<u64>,
    n: u64,
}

impl SimplerPrimeGetter {
    pub fn new() -> Self {
        SimplerPrimeGetter {
            primes: vec![2],
            n: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.primes.len()
    }

    pub fn current_max(&self) -> Option<u64> {
        let res = self.primes[self.len() - 1];
        Some(res)
    }

    pub fn add_prime(&mut self, n: u64) {
        self.primes.push(n);
        self.n += 1;
    }

    pub fn is_prime(&self, candidate_prime: u64) -> bool {
        self.primes.iter().all(|prime| candidate_prime % prime != 0)
    }

    pub fn nth_prime(&mut self, n: u64) -> Option<u64> {
        if n <= 0 {
            return None;
        } else if n == 1 {
            return Some(2);
        } else {
            return self.nth((n - 2).try_into().unwrap());
        }
    }
}

impl Iterator for SimplerPrimeGetter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.current_max().unwrap()..).find(|x| self.is_prime(*x));
        match result {
            Some(p) => {
                self.add_prime(p);
                return Some(p);
            }
            None => {
                println!("I don't think this is possible unless I screwed up.");
                return None;
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn can_make_object() {
        let actual = SimplerPrimeGetter::new();
        let expected = SimplerPrimeGetter {
            primes: vec![2],
            n: 1,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_get_len_of_primes() {
        let p = SimplerPrimeGetter::new();

        assert_eq!(p.len(), 1);
    }

    #[test]
    fn can_push_new_prime() {
        let mut p = SimplerPrimeGetter::new();
        p.add_prime(3);

        assert_eq!(p.len(), 2);
    }

    #[test]
    fn can_test_next_number_to_see_if_it_is_prime() {
        let mut p = SimplerPrimeGetter::new();

        assert!(p.is_prime(3));

        p.add_prime(3);

        assert!(!p.is_prime(4));
    }

    #[test]
    fn can_get_current_max_prime() {
        let mut p = SimplerPrimeGetter::new();

        assert_eq!(p.current_max().unwrap(), 2);

        p.add_prime(3);

        assert_eq!(p.current_max().unwrap(), 3);
    }

    #[test]
    fn can_get_next_prime() {
        let mut p = SimplerPrimeGetter::new();

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
        let mut p = SimplerPrimeGetter::new();
        assert_eq!(p.nth(0).unwrap(), 3);
    }

    #[test]
    fn test_nth_prime() {
        let mut p = SimplerPrimeGetter::new();
        assert_eq!(p.nth_prime(1), Some(2));
        let mut p = SimplerPrimeGetter::new();
        assert_eq!(p.nth_prime(2), Some(3));
        let mut p = SimplerPrimeGetter::new();
        assert_eq!(p.nth_prime(3), Some(5));
        let mut p = SimplerPrimeGetter::new();
        assert_eq!(p.nth_prime(4), Some(7));
        let mut p = SimplerPrimeGetter::new();
        assert_eq!(p.nth_prime(5), Some(11));
        let mut p = SimplerPrimeGetter::new();
        assert_eq!(p.nth_prime(6), Some(13));
    }
}
