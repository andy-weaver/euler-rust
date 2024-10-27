use prime_10001::simpler_prime_getter::SimplerPrimeGetter;

fn main() {
    let mut p2 = SimplerPrimeGetter::new();

    for i in (9999..10002) {
        println!("Prime {}: {:#?}", i + 1, p2.primes[i]);
    }
}
