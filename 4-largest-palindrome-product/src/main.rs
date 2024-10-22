use std::collections::HashSet;

fn is_palindrome(n: impl std::string::ToString) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let mut result_set: HashSet<i32> = HashSet::new();

    (100..=999).rev().for_each(|x1| {
        (100..=999).rev().for_each(|x2| {
            let prod = x1 * x2;

            match is_palindrome(prod) {
                true => {
                    _ = result_set.insert(prod);
                }
                false => (),
            }
        })
    });

    let max_in_set = result_set.iter().max().unwrap();

    println!("The max palindrome is {}", max_in_set);
}
