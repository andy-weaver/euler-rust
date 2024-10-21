use std::collections::{hash_set::Difference, HashSet};

const NONPRIME_NUMBER: i64 = 600851475143;

fn main() {
    let sqrt = (NONPRIME_NUMBER as f64).powf(0.5).round() + 1.0;

    let divisors: Vec<usize> = (3..=(sqrt as usize))
        .filter(|x| NONPRIME_NUMBER % (*x as i64) == 0)
        .to_owned()
        .rev()
        .collect();

    let mut todrop = vec![];
    for d1 in &divisors {
        for d2 in &divisors {
            if d1 > d2 && d1 % d2 == 0 && !todrop.contains(d1) {
                todrop.push(*d1);
            }
        }
    }

    let mut divset: HashSet<usize, _> = HashSet::new();
    let mut todropset: HashSet<usize, _> = HashSet::new();

    divisors.iter().for_each(|x| {
        divset.insert(*x);
    });

    todrop.iter().for_each(|x| {
        todropset.insert(*x);
    });

    let result_set = divset.difference(&todropset).max().unwrap();

    println!("Divisors: {:#?}", divisors);

    println!("Divisors to drop: {:#?}", todrop);

    println!("Largest prime factor:\n{:#?}", result_set);
}
