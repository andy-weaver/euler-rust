use std::ops::Index;

use ndarray::{array, Array2};

#[allow(clippy::zero_prefixed_literal)]
pub fn get_array() -> Array2<u32> {
    array![
        [08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08],
        [49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00],
        [81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65],
        [52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91],
        [22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80],
        [24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50],
        [32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70],
        [67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21],
        [24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72],
        [21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95],
        [78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92],
        [16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57],
        [86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58],
        [19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40],
        [04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66],
        [88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69],
        [04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36],
        [20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16],
        [20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54],
        [01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48]
    ]
}

fn idx_to_usize(idx: i32, n: usize) -> usize {
    if idx >= 0 {
        idx as usize
    } else {
        n - (-idx as usize)
    }
}

pub enum ProductDirection {
    Right,
    Down,
    RightDiagonal,
    LeftDiagonal,
}

#[derive(Debug, PartialEq)]
pub struct GridProduct {
    pub arr: Array2<u32>,
    pub current_max: u64,
    /*     pub current_max_row: u64;
    pub current_max_col: u64; */
}

impl GridProduct {
    pub fn new(arr: Array2<u32>) -> Self {
        GridProduct {
            arr,
            current_max: 0,
        }
    }

    pub fn row(&self, row_num: usize) -> Vec<u32> {
        let res = self.arr.outer_iter().nth(row_num);

        match res {
            Some(arr) => arr.to_vec(),
            None => {
                vec![]
            }
        }
    }

    pub fn right(&self, row: i32, col: i32) -> Option<[u32; 4]> {
        if row <= 19 && col <= 19 - 3 {
            Some([
                self[[row, col]],
                self[[row, col + 1]],
                self[[row, col + 2]],
                self[[row, col + 3]],
            ])
        } else {
            None
        }
    }

    pub fn down(&self, row: i32, col: i32) -> Option<[u32; 4]> {
        if row <= 19 - 3 && col <= 19 {
            Some([
                self[[row, col]],
                self[[row + 1, col]],
                self[[row + 2, col]],
                self[[row + 3, col]],
            ])
        } else {
            None
        }
    }

    pub fn left_diag(&self, row: i32, col: i32) -> Option<[u32; 4]> {
        if row <= 19 - 3 && col <= 19 - 3 {
            Some([
                self[[row + 3, col]],
                self[[row + 2, col + 1]],
                self[[row + 1, col + 2]],
                self[[row, col + 3]],
            ])
        } else {
            None
        }
    }

    pub fn right_diag(&self, row: i32, col: i32) -> Option<[u32; 4]> {
        if row <= 19 - 3 && col <= 19 - 3 {
            Some([
                self[[row, col]],
                self[[row + 1, col + 1]],
                self[[row + 2, col + 2]],
                self[[row + 3, col + 3]],
            ])
        } else {
            None
        }
    }

    pub fn products_at(&self, row: i32, col: i32) -> u64 {
        let result_as_arr_of_arrs = [
            self.right(row, col),
            self.down(row, col),
            self.left_diag(row, col),
            self.right_diag(row, col),
        ];

        let max_products_from_result = result_as_arr_of_arrs
            .iter()
            .map(|&arr| arr.unwrap_or([1, 1, 1, 1]))
            .map(|arr| (0..4).fold(1, |acc, i| acc * arr[i]))
            .map(|product| product as u64)
            .max();

        max_products_from_result.unwrap_or(1)
    }

    pub fn compare_products_with_current_max(&mut self, row: i32, col: i32) {
        let max_product_from_index = self.products_at(row, col);
        if self.current_max < max_product_from_index {
            self.current_max = max_product_from_index
        }
    }
}

impl Index<[i32; 2]> for GridProduct {
    type Output = u32;

    fn index(&self, index: [i32; 2]) -> &Self::Output {
        let row = idx_to_usize(index[0], self.arr.nrows());
        let col = idx_to_usize(index[1], self.arr.ncols());
        &self.arr[[row, col]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_gp() -> GridProduct {
        GridProduct::new(get_array())
    }

    #[test]
    fn can_create_grid_product() {
        let gp1 = GridProduct {
            arr: get_array(),
            current_max: 0,
        };
        let gp2 = GridProduct::new(get_array());

        assert_eq!(gp1, gp2);
    }

    #[test]
    fn can_get_row_by_index() {
        let gp = setup_gp();
        let expected = vec![
            49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 4, 56, 62, 00,
        ];

        let actual = gp.row(1);

        assert_eq!(actual, expected);

        let expected = vec![
            67, 26, 20, 68, 2, 62, 12, 20, 95, 63, 94, 39, 63, 8, 40, 91, 66, 49, 94, 21,
        ];
        let actual = gp.row(7);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_can_index() {
        let gp = setup_gp();
        let actual = gp[[0, 0]];
        let expected = 8;

        assert_eq!(actual, expected);

        let actual = gp[[13, 1]];
        let expected = 80;

        assert_eq!(actual, expected);
    }

    #[test]
    fn can_use_negative_indices_in_rows() {
        let gp = setup_gp();
        assert_eq!(gp[[-1, 0]], 1);
        assert_eq!(gp[[-2, 2]], 35);
        assert_eq!(gp[[-4, 5]], 25);
    }

    #[test]
    fn can_use_negative_indices_in_cols() {
        let gp = setup_gp();
        assert_eq!(gp[[0, -1]], 8);
        assert_eq!(gp[[10, -3]], 53);
    }

    #[test]
    fn can_get_4_numbers_after_index_number() {
        let gp = setup_gp();
        let next4 = gp.right(0, 0);

        assert_eq!(next4, Some([8, 2, 22, 97]));

        let next4 = gp.right(19, 16);
        assert_eq!(next4, Some([89, 19, 67, 48]));

        let oob_next4 = gp.right(19, 17);
        assert!(oob_next4.is_none());
    }

    #[test]
    fn can_get_4_numbers_below_index_number() {
        let gp = setup_gp();
        let next4 = gp.down(0, 0);

        assert_eq!(next4, Some([8, 49, 81, 52]));

        let next4 = gp.down(16, 19);
        assert_eq!(next4, Some([36, 16, 54, 48]));
    }

    #[test]
    fn down_returns_none_when_row_gt16() {
        let gp = setup_gp();

        let oob_next4 = gp.down(17, 19);
        assert!(oob_next4.is_none());

        let oob_next4 = gp.down(16, 19);
        assert!(oob_next4.is_some());
    }

    #[test]
    fn can_get_4_numbers_right_diag_from_index_number() {
        let gp = setup_gp();
        let next4 = gp.right_diag(0, 0);
        let expected = Some([8, 49, 31, 23]);

        assert_eq!(next4, expected);

        let next4 = gp.right_diag(16, 16);
        let expected = Some([40, 4, 5, 48]);

        assert_eq!(next4, expected);
    }

    #[test]
    fn right_diag_returns_none_when_either_row_or_col_gt16() {
        let gp = setup_gp();

        let oob_next4 = gp.right_diag(17, 1);
        assert!(oob_next4.is_none());

        let oob_next4 = gp.right_diag(1, 18);
        assert!(oob_next4.is_none());

        let oob_next4 = gp.right_diag(19, 18);
        assert!(oob_next4.is_none());
    }

    #[test]
    fn can_get_4_numbers_left_diag_from_index_number() {
        let gp = setup_gp();

        assert_eq!(gp.left_diag(0, 0), Some([52, 49, 99, 97]));
        assert_eq!(gp.left_diag(16, 16), Some([89, 57, 36, 36]));
    }

    #[test]
    fn left_diag_returns_none_when_either_row_or_col_gt16() {
        let gp = setup_gp();

        let oob_next4 = gp.left_diag(17, 1);
        assert!(oob_next4.is_none());

        let oob_next4 = gp.left_diag(1, 18);
        assert!(oob_next4.is_none());

        let oob_next4 = gp.left_diag(19, 18);
        assert!(oob_next4.is_none());
    }
}
