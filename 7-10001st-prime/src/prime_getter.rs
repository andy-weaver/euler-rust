// use std::iter::Iterator;

pub type PrimesVec = Vec<u64>;

pub struct PrimeGetter {
    primes: PrimesVec,
}

impl PrimeGetter {
    pub fn new() -> Self {
        PrimeGetter { primes: vec![2] }
    }

    pub fn primes_as_ref(&self) -> &PrimesVec {
        &self.primes
    }

    pub fn len(&self) -> usize {
        self.primes.len()
    }

    pub fn largest(&self) -> &u64 {
        &self.primes[&self.len() - 1]
    }

    pub fn add_prime(&mut self, n: u64) {
        self.primes.push(n);
    }

    /*     pub fn iter(&self) -> std::iter::Iterator {
        &self.primes_as_ref().iter()
    } */

    // fn is_prime(candidate: u64) -> bool {}
}

/* impl Iterator for PrimeGetter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next_item = self.primes.iter().next().copied();
        match &next_item {
            Some(x) => {
                self.primes = self.primes[1..].to_vec();
            }
            None =>
        }
        next_item
    }
} */

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_can_create_from_struct_def() {
        let p = PrimeGetter { primes: vec![2] };
        assert_eq!(p.primes, vec![2]);
    }

    #[test]
    fn test_new_method_is_the_same_as_creating_from_the_def_with_vec2() {
        let p = PrimeGetter::new();
        assert_eq!(p.primes, vec![2]);
    }

    #[test]
    fn test_can_access_ref_to_primes_vec() {
        let p = PrimeGetter::new();
        assert_eq!(p.primes_as_ref(), &vec![2]);
    }

    #[test]
    fn test_can_get_largest_prime_as_ref_from_new_instance() {
        let p = PrimeGetter::new();
        assert_eq!(p.largest(), &2);
    }

    #[test]
    fn test_can_add_to_primes_vec() {
        let mut p = PrimeGetter::new();
        p.add_prime(3);
        assert_eq!(p.primes_as_ref(), &vec![2, 3]);

        p.add_prime(5);
        assert_eq!(p.primes_as_ref(), &vec![2, 3, 5]);
    }

    #[test]
    fn test_can_get_len_of_primes() {
        let p = PrimeGetter::new();
        assert_eq!(p.len(), 1);
    }

    #[test]
    fn test_can_get_largest_prime_after_adding_3() {
        let mut p = PrimeGetter::new();
        p.add_prime(3);

        assert_eq!(p.largest(), &3);
    }

    /*     #[test]
    fn test_can_make_iterator() {
        let mut p = PrimeGetter::new();
        p.add_prime(3);
        p.add_prime(5);
        p.add_prime(7);

        assert_eq!(p.next(), Some(2));
        assert_eq!(p.next(), Some(3));
        assert_eq!(p.next(), Some(5));
        assert_eq!(p.next(), Some(7));
        assert_eq!(p.next(), None);
    } */

    /*     #[test]
    fn test_is_prime() {
        assert!(PrimeGetter::is_prime(2));
        assert!(PrimeGetter::is_prime(3));
        assert!(PrimeGetter::is_prime(5));
        assert!(PrimeGetter::is_prime(7));

        assert!(!PrimeGetter::is_prime(4));
        assert!(!PrimeGetter::is_prime(6));
        assert!(!PrimeGetter::is_prime(8));
        assert!(!PrimeGetter::is_prime(9));
        assert!(!PrimeGetter::is_prime(10));
    } */
}
