use summation_of_primes::PrimeGenerator;

fn main() {
    let max_prime = 2 * 10u64.pow(6); // 2,000,000
    let mut p = PrimeGenerator::new();
    let result = p.sum_primes_below_n(max_prime);
    println!("{}", result);
}

// 203,627,916,956 + 328
