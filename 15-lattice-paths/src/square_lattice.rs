#[derive(Debug, PartialEq)]
pub struct SquareLattice {
    pub size: usize,
    pub lattice: Vec<Vec<u64>>,
    pub current_position: usize,
}

impl SquareLattice {
    pub fn new(size: usize) -> Self {
        let n_nodes_in_row = size + 1;
        let row = (1..=n_nodes_in_row).collect::<Vec<usize>>();
        let mut lattice = Vec::new();
        (0..n_nodes_in_row).for_each(|i| {
            let scaled_row = row
                .iter()
                .map(|x| x + (i * n_nodes_in_row))
                .map(|x| x as u64)
                .collect::<Vec<u64>>();
            lattice.push(scaled_row);
        });
        Self {
            size,
            lattice,
            current_position: 1,
        }
    }

    fn n_nodes_in_row(&self) -> usize {
        self.size + 1
    }

    pub fn get(&self, x: usize, y: usize) -> u64 {
        self.lattice[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u64) {
        self.lattice[x][y] = value;
    }

    pub fn display(&self) {
        for i in 0..self.n_nodes_in_row() {
            for j in 0..self.n_nodes_in_row() {
                print!("{:4}", self.lattice[i][j]);
            }
            println!();
        }
    }

    pub fn is_last_column(&self) -> bool {
        self.current_position % self.n_nodes_in_row() == 0
    }

    pub fn is_last_row(&self) -> bool {
        self.current_position > self.n_nodes_in_row() * self.size
    }

    pub fn move_right(&mut self) -> bool {
        if self.is_last_column() {
            return false;
        }

        self.current_position += 1;
        true
    }

    pub fn move_down(&mut self) -> bool {
        if self.is_last_row() {
            return false;
        }

        self.current_position += self.n_nodes_in_row();
        true
    }
}

#[cfg(test)]

pub mod tests {
    use super::*;

    #[test]
    fn test_square_lattice() {
        let lattice = SquareLattice::new(5);
        lattice.display();
        assert_eq!(lattice.get(0, 0), 1);
        assert_eq!(lattice.get(1, 1), 8);
        assert_eq!(lattice.get(2, 2), 15);
        assert_eq!(lattice.get(3, 3), 22);
        assert_eq!(lattice.get(4, 4), 29);
        assert_eq!(lattice.get(5, 5), 36);
    }

    #[test]
    fn test_new_method_is_same_as_building_struct_directly() {
        let lattice = SquareLattice::new(5);
        let lattice2 = SquareLattice {
            size: 5,
            lattice: vec![
                vec![1, 2, 3, 4, 5, 6],
                vec![7, 8, 9, 10, 11, 12],
                vec![13, 14, 15, 16, 17, 18],
                vec![19, 20, 21, 22, 23, 24],
                vec![25, 26, 27, 28, 29, 30],
                vec![31, 32, 33, 34, 35, 36],
            ],
            current_position: 1,
        };
        assert_eq!(lattice.size, lattice2.size);
        assert_eq!(lattice.lattice, lattice2.lattice);
        assert_eq!(lattice.current_position, lattice2.current_position);
        assert_eq!(lattice, lattice2);
    }

    #[test]
    fn can_get() {
        let lattice = SquareLattice::new(5);
        assert_eq!(lattice.get(0, 0), 1);
        assert_eq!(lattice.get(1, 1), 8);
        assert_eq!(lattice.get(2, 2), 15);
        assert_eq!(lattice.get(3, 3), 22);
        assert_eq!(lattice.get(4, 4), 29);
        assert_eq!(lattice.get(5, 5), 36);
    }

    #[test]
    fn can_set() {
        let mut lattice = SquareLattice::new(5);
        lattice.set(0, 0, 100);
        assert_eq!(lattice.get(0, 0), 100);
    }

    #[test]
    fn from_initial_position_can_move_to_the_right() {
        let mut lattice = SquareLattice::new(5);
        assert_eq!(lattice.current_position, 1);
        lattice.move_right();
        assert_eq!(lattice.current_position, 2);
    }

    #[test]
    fn from_initial_position_can_move_down() {
        let mut lattice = SquareLattice::new(5);
        assert_eq!(lattice.current_position, 1);
        lattice.move_down();
        assert_eq!(lattice.current_position, 7);
    }

    #[test]
    fn from_initial_position_can_move_right_size_times_but_no_more() {
        let mut lattice = SquareLattice::new(5);
        assert_eq!(lattice.current_position, 1);

        assert!(lattice.move_right());
        assert_eq!(lattice.current_position, 2);

        assert!(lattice.move_right());
        assert_eq!(lattice.current_position, 3);

        assert!(lattice.move_right());
        assert_eq!(lattice.current_position, 4);

        assert!(lattice.move_right());
        assert_eq!(lattice.current_position, 5);

        assert!(lattice.move_right());
        assert_eq!(lattice.current_position, 6);

        assert!(!lattice.move_right());

        let mut lattice = SquareLattice::new(2);
        assert!(lattice.move_right());
        assert!(lattice.move_right());
        assert!(!lattice.move_right());
    }

    #[test]
    fn from_initial_position_can_move_down_size_times_but_no_more() {
        let mut lattice = SquareLattice::new(5);
        assert_eq!(lattice.current_position, 1);

        assert!(lattice.move_down());
        assert_eq!(lattice.current_position, 7);

        assert!(lattice.move_down());
        assert_eq!(lattice.current_position, 13);

        assert!(lattice.move_down());
        assert_eq!(lattice.current_position, 19);

        assert!(lattice.move_down());
        assert_eq!(lattice.current_position, 25);

        assert!(lattice.move_down());
        assert_eq!(lattice.current_position, 31);

        assert!(!lattice.move_down());

        let mut lattice = SquareLattice::new(2);
        assert!(lattice.move_down());
        assert!(lattice.move_down());
        assert!(!lattice.move_down());
    }
}
