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
}

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
    fn test_can_access_mut_ref_to_primes_vec() {
        let p = PrimeGetter::new();
        let mut expected = vec![ 2 as u64];

        assert_eq!(p.primes_as_mut_ref(), &expected);
    }

}
