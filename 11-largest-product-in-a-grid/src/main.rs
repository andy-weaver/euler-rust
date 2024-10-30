use largest_product_in_a_grid::{get_array, GridProduct};

fn main() {
    let mut gp = GridProduct::new(get_array());

    for row in 0..20 {
        for col in 0..20 {
            gp.compare_products_with_current_max(row, col);
        }
    }

    println!("Max product: {}", gp.current_max);
}
