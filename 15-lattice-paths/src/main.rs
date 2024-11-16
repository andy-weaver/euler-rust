use lattice_paths::lattice_path::LatticePath;

fn main() {
    // let n_sides = 20;
    // calculate_complete_paths(n_sides);
    // (2..10).for_each(calculate_complete_paths);
    // (2..10).for_each(|x| {
    //     println!(
    //         "{} choose {} = {}",
    //         2 * x,
    //         x,
    //         binomial_coefficient(2 * x, x)
    //     )
    // });

    println!("40 choose 20 = {}", binomial_coefficient(40, 20));
}

fn calculate_complete_paths(n_sides: usize) {
    let path = LatticePath::new(n_sides);
    let n_complete_paths = path.n_complete_paths();

    println!(
        "Number of complete paths in a {}x{} lattice: {}",
        n_sides, n_sides, n_complete_paths
    );
}

fn binomial_coefficient(n: usize, k: usize) -> usize {
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }

    result
}
