#![allow(dead_code)]

struct Solution;

impl Solution {
    /// You are given an n x n 2D matrix representing an image.
    ///
    /// Rotate the image by 90 degrees (clockwise).
    ///
    /// **Note:**
    ///
    /// You have to rotate the image in-place, which means you have to modify the input
    /// 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Solution::transpose(matrix);

        // Reverse the columns
        matrix
            .iter_mut()
            .for_each(|row| row.reverse());
    }

    fn transpose(matrix: &mut Vec<Vec<i32>>) {
        for row_index in 0..matrix.len() {
            // We only take the columns after `row_index` because
            // we've already rotated the ones before the row index
            //
            // Our iteration pattern looks like this:
            //
            // IIIII
            //  IIII
            //   III
            //    II
            //     I
            //
            for col_index in row_index..matrix.len() {
                let target_row = col_index;
                let target_col = row_index;

                let temp: i32 = matrix[target_row][target_col];
                matrix[target_row][target_col] = matrix[row_index][col_index];
                matrix[row_index][col_index] = temp;
            }
        }
    }

    pub fn rotate_no_transpose(matrix: &mut Vec<Vec<i32>>) {
        let mut row = 0;
        let mut col_start = 0;
        let (mut col_end, mut overflow) = matrix.len().overflowing_sub(2);

        while !overflow && col_start <= col_end {
            for column in col_start..=col_end {
                let (target_row, target_col) = Solution::rotate_point(matrix, row, column);
                let previous_value = matrix[target_row][target_col];
                matrix[target_row][target_col] = matrix[row][column];

                let (target_row, target_col) = Solution::rotate_point(matrix, target_row, target_col);
                let previous_value_2 = matrix[target_row][target_col];
                matrix[target_row][target_col] = previous_value;

                let (target_row, target_col) = Solution::rotate_point(matrix, target_row, target_col);
                let previous_value = matrix[target_row][target_col];
                matrix[target_row][target_col] = previous_value_2;

                let (target_row, target_col) = Solution::rotate_point(matrix, target_row, target_col);
                matrix[target_row][target_col] = previous_value;
            }

            row += 1;
            col_start += 1;

            let (col_end_value, overflow_value) = col_end.overflowing_sub(1);
            col_end = col_end_value;
            overflow = overflow_value;
        }
    }

    fn rotate_point(
        matrix: &Vec<Vec<i32>>,
        row: usize,
        col: usize
    ) -> (usize, usize) {
        let target_row = col;
        let target_col = matrix.len() - row - 1;

        (target_row, target_col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_all_solutions(input: Vec<Vec<i32>>, expected_output: Vec<Vec<i32>>) {
        test_solution(&input, &expected_output, Solution::rotate);
        test_solution(&input, &expected_output, Solution::rotate_no_transpose);
    }

    pub fn test_solution(
        input: &Vec<Vec<i32>>,
        expected_output: &Vec<Vec<i32>>,
        solution: fn(&mut Vec<Vec<i32>>)
    ) {
        let mut input = input.clone();
        solution(&mut input);
        assert_eq!(input, expected_output.clone());
    }

    /// Given input matrix:
    ///
    /// ```text
    /// [
    ///   [1,2,3],
    ///   [4,5,6],
    ///   [7,8,9]
    /// ]
    /// ```
    ///
    /// Rotate the input matrix in-place such that it becomes:
    ///
    /// ```text
    /// [
    ///   [7,4,1],
    ///   [8,5,2],
    ///   [9,6,3]
    /// ]
    /// ```
    ///
    #[test]
    pub fn example_1() {
        let input = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ];

        let expected_output = vec![
            vec![7,4,1],
            vec![8,5,2],
            vec![9,6,3],
        ];

        test_all_solutions(input, expected_output);
    }

    /// Given input matrix:
    /// ```text
    /// [
    ///   [ 5, 1, 9,11],
    ///   [ 2, 4, 8,10],
    ///   [13, 3, 6, 7],
    ///   [15,14,12,16]
    /// ],
    /// ```
    ///
    /// Rotate the input matrix in-place such that it becomes:
    /// ```text
    /// [
    ///   [15,13, 2, 5],
    ///   [14, 3, 4, 1],
    ///   [12, 6, 8, 9],
    ///   [16, 7,10,11]
    /// ]
    /// ```
    #[test]
    pub fn example_2() {
        let input = vec![
            vec![ 5, 1, 9,11],
            vec![ 2, 4, 8,10],
            vec![13, 3, 6, 7],
            vec![15,14,12,16],
        ];

        let expected_output = vec![
            vec![15,13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7,10,11]
        ];

        test_all_solutions(input, expected_output);
    }

    #[test]
    pub fn empty() {
        test_all_solutions(Vec::new(), Vec::new());
    }

    #[test]
    pub fn one_input() {
        test_all_solutions(vec![vec![1]], vec![vec![1]])
    }

    #[test]
    pub fn two_by_two() {
        let input = vec![
            vec![1,2],
            vec![3,4]
        ];

        let expected_output = vec![
            vec![3,1],
            vec![4,2],
        ];

        test_all_solutions(input, expected_output);
    }
}
