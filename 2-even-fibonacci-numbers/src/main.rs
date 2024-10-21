fn fibonacci_number(n: i64) -> i64 {
    if n < 0 {
        return 0;
    }
    if n < 2 {
        return 1;
    }

    fibonacci_number(n - 2) + fibonacci_number(n - 1)
}

fn main() {
    let max_value = 4000000;

    let result = (0..=max_value)
        .into_iter()
        .reduce(|mut acc, x| acc + fibonacci_number(x))
        .unwrap();
    println!("{}", result);
}
