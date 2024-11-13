use large_sum::vector::{
    break_x_into_blocks, first_10_digits_of_sum, get_large_number_vec, sum_parts,
};

fn main() {
    let large_vec = get_large_number_vec();
    let large_vec_to_parts = large_vec
        .iter()
        .map(|x| break_x_into_blocks(x.to_owned()))
        .collect::<Vec<(u64, u64, u64, u64)>>();
    let result = first_10_digits_of_sum(large_vec_to_parts);
    println!("{}", result);
}
