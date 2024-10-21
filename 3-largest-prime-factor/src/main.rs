const NONPRIME_NUMBER: i64 = 600851475143;



fn main() {
    let sqrt = (NONPRIME_NUMBER as f64).powf(0.5).round() + 1.0;

    let divisors: Vec<_> = (3..=(sqrt as usize))
        .filter(|x| NONPRIME_NUMBER % (*x as i64) == 0)
        .collect();

    let d2 = divisors.iter().

    println!("Divisors: {:#?}", divisors);
}
