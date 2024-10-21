#[derive(Debug, Clone, PartialEq)]
struct Fibonacci {
    n: i64,
    fibs: Vec<i64>,
    evens: Vec<i64>,
}

impl Fibonacci {
    fn new() -> Self {
        let mut fibs = Vec::with_capacity(50);
        fibs.push(1);
        fibs.push(1);
        Fibonacci {
            n: 2,
            fibs,
            evens: Vec::with_capacity(50),
        }
    }

    fn next(&mut self) {
        let n1 = self.fibs.len() - 2;
        let last2: i64 = self
            .fibs
            .iter()
            .enumerate()
            .filter(|(i, _)| i >= &n1)
            .map(|(_, x)| x)
            .sum();
        if &last2 % 2 == 0 {
            println!("Pushing fib({})={}.", self.n, &last2);
            self.evens.push(last2)
        }

        self.fibs.push(last2);
        self.n += 1;
    }

    fn current(&self) -> i64 {
        self.fibs.last().unwrap().to_owned()
    }

    fn sum_evens(&self) -> i64 {
        self.evens.iter().sum()
    }
}

fn main() {
    let max_value = 4000000;

    let mut f = Fibonacci::new();

    while f.current() <= max_value {
        f.next()
    }

    println!("After {} iterations, the sum is {}", f.n, f.sum_evens())
}
