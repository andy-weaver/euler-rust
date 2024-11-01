use highly_divisible_triangular_number::triangular_number_maker::TriangularNumberMaker;
fn main() {
    let mut t = TriangularNumberMaker::new();

    (0..100).for_each(|_| {
        t.next();
    });
    println!("Triangular Numbers: {:#?}", t.numbers);
}
