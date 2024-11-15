use longest_collatz_sequence::collatz_generator::get_collatz_sequence_len;
// use rayon::prelude::*;
use std::sync::Mutex;

fn main() {
    let start = 1;
    let end = 1_000_000;
    let result = Mutex::new(0);
    println!(
        "Calculating the longest Collatz sequence length from {} to {}",
        start, end
    );
    print!("0");

    (start..=end)
        // .into_par_iter()
        .for_each(|n| {
            let len = get_collatz_sequence_len(n);
            let mut result = result.lock().unwrap();
            if len > *result {
                *result = len;
                print!(" -> {} ({})", len, n);
            }
        });
}
