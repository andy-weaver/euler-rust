use multiples_of_3_or_5::MultiplesOf;

fn main() {
    let mut m3 = MultiplesOf::new(3);
    let mut m5 = MultiplesOf::new(5);
    let mut m15 = MultiplesOf::new(15);

    m3.get_multiples_below(1000);
    m5.get_multiples_below(1000);
    m15.get_multiples_below(1000);

    let answer = m3.sum_multiples() + m5.sum_multiples() - m15.sum_multiples();
    println!("The sum of multiples of 3 or 5 under 1000 is:\n{}", answer);
}
