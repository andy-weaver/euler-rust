use prime_10001::simpler_prime_getter::SimplerPrimeGetter;

fn main() {
    // let mut p = SimplerPrimeGetter::new();
    let mut p2 = SimplerPrimeGetter::new();

    /*     loop {
        p.next();

        if p.len() == 10001 {
            break;
        } else if p.len() % 1000 == 0 {
            println!("{} Primes...", p.len());
        }
    }

    let res = p.current_max();

    println!("The 10001st prime is {}.", res.unwrap()); */
    println!("{}", p2.nth(10001).unwrap());
    for i in (9990..) {
        println!("Prime {}: {:#?}", i+1, p2.primes[i]);
    }
}
