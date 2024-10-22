fn main() {
    let primes = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let additional_prime_factors = vec![
        2, 2, 2, // need at least 4 2's to be divisible by 16
        3, // need at least 2 3's to be divisible by 9
    ];

    let result = primes.iter().fold(1, |mut acc, x| acc * x);
    println!("Prime product: {}", &result);
    println!("Additional factors: {}", 2 * 2 * 2 * 3);
    println!("Total product: {} x {} = {}", &result, 8 * 3, result * 24);
}
