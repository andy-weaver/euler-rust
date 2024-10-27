use largest_product_in_a_series::{find_max_product, Windower};

fn main() {
    let mut w = Windower::new(13);
    let res = find_max_product(&mut w);

    println!("Largest {}-digit product is: {}", w.get_window_size(), res);
}
