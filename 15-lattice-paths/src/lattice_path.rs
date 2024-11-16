use crate::square_lattice::SquareLattice;

#[derive(Debug, PartialEq)]
pub struct LatticePath {
    pub lattice: SquareLattice,
    pub path: Vec<u64>,
}

impl LatticePath {
    pub fn new(size: usize) -> Self {
        let lattice = SquareLattice::new(size);
        let path = vec![1];
        Self { lattice, path }
    }

    pub fn move_down(&mut self) -> bool {
        if !self.lattice.is_last_row() {
            self.lattice.move_down();
            self.path.push(self.lattice.current_position as u64);
            true
        } else {
            false
        }
    }

    pub fn move_right(&mut self) -> bool {
        if !self.lattice.is_last_column() {
            self.lattice.move_right();
            self.path.push(self.lattice.current_position as u64);
            true
        } else {
            false
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.lattice.is_last_column() && self.lattice.is_last_row()
    }

    pub fn total_path_size_needed(&self) -> usize {
        self.lattice.size * 2 + 1 // add 1 for the initial position
    }

    pub fn len(&self) -> usize {
        self.path.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_path_complete(&self) -> bool {
        self.len() == self.total_path_size_needed()
    }

    pub fn from_path_vec(path_vec: Vec<u8>, size: usize) -> Result<Self, String> {
        let mut path = LatticePath::new(size);

        if path_vec.len() != path.total_path_size_needed() - 1 {
            return Err(format!(
                "Path vector must be of length {}, but is of length {}",
                path.total_path_size_needed() - 1,
                path_vec.len()
            ));
        }

        path_vec.iter().for_each(|&step| {
            match step {
                0 => path.move_down(),
                1 => path.move_right(),
                _ => panic!("Invalid step in path_vec"),
            };
        });

        Ok(path)
    }

    pub fn generate_all_possible_path_vecs(&self) -> Vec<Vec<u8>> {
        let mut result = Vec::new();
        let n_moves = self.total_path_size_needed() - 1;

        for i in 0..2_u64.pow(n_moves as u32) {
            let mut path_vec = Vec::new();
            let mut i = i;
            let mut n_downs = 0;
            let mut n_rights = 0;
            for _ in 0..n_moves {
                path_vec.push((i % 2) as u8);
                if i % 2 == 0 {
                    n_downs += 1;
                } else {
                    n_rights += 1;
                }

                if n_downs > self.lattice.size || n_rights > self.lattice.size {
                    break;
                }
                i /= 2;
            }
            if n_downs != n_rights {
                continue;
            }
            path_vec.reverse();
            result.push(path_vec);
        }

        result
    }

    pub fn n_complete_paths(&self) -> u64 {
        let mut n_complete_paths = 0;
        let path_vecs = self.generate_all_possible_path_vecs();

        path_vecs.iter().for_each(|path_vec| {
            if let Ok(path) = LatticePath::from_path_vec(path_vec.clone(), self.lattice.size) {
                if path.is_path_complete() {
                    n_complete_paths += 1;
                }
            }
        });

        n_complete_paths
    }
}

#[cfg(test)]

pub mod tests {
    use super::*;

    #[test]
    fn can_make_lattice_path() {
        let path = LatticePath::new(2);
        let test_square_lattice = SquareLattice::new(2);
        assert_eq!(path.lattice, test_square_lattice);
        assert_eq!(path.path, vec![1]);

        let test_path = LatticePath {
            lattice: SquareLattice::new(2),
            path: vec![1],
        };

        assert_eq!(path, test_path);
    }

    #[test]
    fn can_move_down() {
        let mut path = LatticePath::new(2);
        assert!(path.move_down());
        assert_eq!(path.path, vec![1, 4]);
        assert!(path.move_down());
        assert_eq!(path.path, vec![1, 4, 7]);
        assert!(!path.move_down());
    }

    #[test]
    fn can_move_right() {
        let mut path = LatticePath::new(2);
        assert!(path.move_right());
        assert_eq!(path.path, vec![1, 2]);
        assert!(path.move_right());
        assert_eq!(path.path, vec![1, 2, 3]);
        assert!(!path.move_right());
    }

    #[test]
    fn can_move_down_and_right() {
        let mut path = LatticePath::new(2);
        assert!(path.move_down());
        assert!(path.move_right());
        assert_eq!(path.path, vec![1, 4, 5]);
        assert!(path.move_down());
        assert!(path.move_right());
        assert_eq!(path.path, vec![1, 4, 5, 8, 9]);
        assert!(!path.move_down());
        assert!(!path.move_right());
    }

    #[test]
    fn can_move_right_and_down() {
        let mut path = LatticePath::new(2);
        assert!(path.move_right());
        assert!(path.move_down());
        assert_eq!(path.path, vec![1, 2, 5]);
        assert!(path.move_right());
        assert!(path.move_down());
        assert_eq!(path.path, vec![1, 2, 5, 6, 9]);
        assert!(!path.move_right());
        assert!(!path.move_down());
    }

    #[test]
    fn can_tell_if_at_the_end_of_the_lattice() {
        let mut path = LatticePath::new(2);
        assert!(!path.is_at_end());
        path.move_down();
        assert!(!path.is_at_end());
        path.move_down();
        assert!(!path.is_at_end());

        path.move_right();
        assert!(!path.is_at_end());
        path.move_right();
        assert!(path.is_at_end());
    }

    #[test]
    fn can_get_path_size_needed_to_get_to_the_end() {
        let path = LatticePath::new(2);
        assert_eq!(path.total_path_size_needed(), 5);
    }

    #[test]
    fn can_get_path_size_needed_to_get_to_the_end_of_a_3x3_lattice() {
        let path = LatticePath::new(3);
        assert_eq!(path.total_path_size_needed(), 7);
    }

    #[test]
    fn can_get_path_len() {
        let mut path = LatticePath::new(2);
        assert_eq!(path.len(), 1);
        path.move_down();
        assert_eq!(path.len(), 2);
        path.move_right();
        assert_eq!(path.len(), 3);
        path.move_down();
        assert_eq!(path.len(), 4);
        path.move_right();
        assert_eq!(path.len(), 5);
    }

    #[test]
    fn can_tell_if_path_is_empty() {
        let path = LatticePath::new(2);
        assert!(!path.is_empty());
        let mut path = LatticePath::new(2);
        path.move_down();
        path.move_right();
        path.move_down();
        path.move_right();
        path.move_down();
        path.move_right();
        assert!(!path.is_empty());
    }

    #[test]
    fn can_tell_if_path_is_complete() {
        let mut path = LatticePath::new(2);
        assert!(!path.is_path_complete());
        path.move_down();
        assert!(!path.is_path_complete());
        path.move_right();
        assert!(!path.is_path_complete());
        path.move_down();
        assert!(!path.is_path_complete());
        path.move_right();
        assert!(path.is_path_complete());
    }

    #[test]
    fn can_convert_vec_of_u8s_to_path() {
        let path_vec: Vec<u8> = vec![0, 1, 0, 1];

        let path1 = LatticePath::from_path_vec(path_vec, 2);
        let mut path2 = LatticePath::new(2);
        path2.move_down();
        path2.move_right();
        path2.move_down();
        path2.move_right();

        assert_eq!(path1.unwrap(), path2);
    }

    #[test]
    fn can_generate_all_possible_path_vecs() {
        let path = LatticePath::new(2);
        let path_vecs = path.generate_all_possible_path_vecs();

        let n_moves = path.total_path_size_needed() - 1;

        // for each move, there are two possibilities: down or right
        assert_eq!(path_vecs.len(), 2_u64.pow(n_moves as u32) as usize);
    }

    #[test]
    fn can_return_the_number_of_complete_paths() {
        let path = LatticePath::new(2);
        let n_complete_paths = path.n_complete_paths();
        let expected = 6;
        assert_eq!(n_complete_paths, expected);
    }
}
