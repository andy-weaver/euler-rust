use sum_square_difference::{square_of_sums, sum_of_squares};

fn main() {
    let sum_of_squares_100 = sum_of_squares(100);
    let square_of_sums_100 = square_of_sums(100);

    let result = &sum_of_squares_100 - &square_of_sums_100;
    println!("Sum of squares: {}", &sum_of_squares_100);
    println!("Square of sums: {}", &square_of_sums_100);
    println!(
        "{} - {} = {}",
        sum_of_squares_100, square_of_sums_100, result
    );
}
