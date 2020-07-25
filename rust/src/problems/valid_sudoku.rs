#![allow(dead_code)]

struct Solution;

impl Solution {
    /// Determine if a 9x9 Sudoku board is valid. Only the filled cells need to be
    /// validated according to the following rules:
    ///
    /// - Each row must contain the digits 1-9 without repetition.
    /// - Each column must contain the digits 1-9 without repetition.
    /// - Each of the 3x3 sub-boxes must contain the digits 1-9 without repetition.
    ///
    /// **Notes:**
    ///
    /// - The given board contain only digits 1-9 and the character '.'.
    /// - A Sudoku board (partially filled) could be valid but is not necessarily solvable.
    /// - Only the filled cells need to be validated according to the mentioned rules.
    /// - The given board size is always 9x9.
    ///
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in 0..9 {
            if !Solution::unique_values(Solution::row(&board, row)) {
                return false;
            }
        }

        for col in 0..9 {
            if !Solution::unique_values(Solution::column(&board, col)) {
                return false;
            }
        }

        for x in 0..3 {
            for y in 0..3 {
                if !Solution::unique_values(Solution::square(&board, x, y)) {
                    return false;
                }
            }
        }

        true
    }

    fn unique_values(items: impl Iterator<Item = char>) -> bool {
        let mut uniques = std::collections::HashSet::new();
        items.into_iter()
            .filter(|&item| item != '.')
            .all(|item| uniques.insert(item))
    }

    /// Iterate over a sudoku row. The top row is row 0, the bottom row is row 8
    fn row<'a>(board: &'a Vec<Vec<char>>, row_number: usize) -> impl Iterator<Item = char> + 'a {
        board[row_number].iter().cloned()
    }

    /// Iterate over a sudoku column. The leftmost row is row 0, the rightmost row is row 8.
    fn column<'a>(board: &'a Vec<Vec<char>>, column_number: usize) -> impl Iterator<Item = char> + 'a {
        (0..9).map(move |row_number| board[row_number][column_number])
    }

    /// Iterate over a sudoku square. The top left square is (0, 0). Bottom right is (2, 2).
    ///
    /// Positions:
    ///
    /// ```text
    /// (0,0) (1,0) (2,0)
    /// (0,1) (1,1) (2,1)
    /// (0,2) (1,2) (2,2)
    /// ```
    fn square<'a>(board: &'a Vec<Vec<char>>, square_x: usize, square_y: usize) -> impl Iterator<Item = char> + 'a {
        let positions = vec![
            (0,0), (1,0), (2,0),
            (0,1), (1,1), (2,1),
            (0,2), (1,2), (2,2)
        ];

        let offset_x = square_x * 3;
        let offset_y = square_y * 3;

        positions
            .into_iter()
            .map(move |(x, y)| board[offset_y + y][offset_x + x])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example_1() {
        let input = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9'],
        ];

        assert_eq!(Solution::is_valid_sudoku(input), true);
    }

    #[test]
    pub fn example_2() {
        let input = vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9'],
        ];

        assert_eq!(Solution::is_valid_sudoku(input), false);
    }

    #[test]
    pub fn example_3() {
        let input = vec![
            vec!['.','.','.','.','5','.','.','1','.'],
            vec!['.','4','.','3','.','.','.','.','.'],
            vec!['.','.','.','.','.','3','.','.','1'],
            vec!['8','.','.','.','.','.','.','2','.'],
            vec!['.','.','2','.','7','.','.','.','.'],
            vec!['.','1','5','.','.','.','.','.','.'],
            vec!['.','.','.','.','.','2','.','.','.'],
            vec!['.','2','.','9','.','.','.','.','.'],
            vec!['.','.','4','.','.','.','.','.','.'],
        ];

        assert_eq!(Solution::is_valid_sudoku(input), false);
    }

}
