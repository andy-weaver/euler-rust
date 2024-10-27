use special_pythagorean_triplet::Triplet;

pub mod tests;
fn main() {
    for a in 1..1000 {
        for b in 1..1000 {
            let t = Triplet::from_ab(a, b);

            if t.is_ordered_correctly() && t.is_pythagorean_triple() && t.is_sum_abc_equal_to_1000()
            {
                println!("a: {}", t.a);
                println!("b: {}", t.b);
                println!("c: {}", t.c);
                println!("Product: {}", t.a * t.b * t.c);
            }
        }
    }
}
