use large_sum::add_two_numbers::NumVec;
use large_sum::vector::get_large_number_vec;

fn main() {
    let large_number_vec = get_large_number_vec();
    let mut large_number_numvec = Vec::new();

    (0..large_number_vec.len()).for_each(|i| {
        let num_vec = NumVec::new(large_number_vec[i].to_string());
        large_number_numvec.push(num_vec);
    });

    let result = NumVec::add_digits(large_number_numvec).0;
    println!("{}", result);
    println!("First 10 digits: {}", &result[..10]);
}
