fn main() {
    let input = include_str!("input");
    println!("First part: {}", first_part(input));
    println!("Second part: {}", second_part(input));
}

trait Grid {
    fn rows(&self) -> &Vec<Vec<char>>;
    fn cols(&self) -> Vec<Vec<char>>;
    fn diagonals(&self) -> Vec<Vec<char>>;
    fn cnt_columns(&self) -> usize;
    fn cnt_rows(&self) -> usize;
}

struct GridScanner<'g> {
    curr_row: usize,
    curr_col: usize,
    grid: &'g Vec<Vec<char>>,
}

impl<'g> GridScanner<'g> {
    fn new(grid: &'g Vec<Vec<char>>) -> Self {
        Self {
            curr_col: 0,
            curr_row: 0,
            grid,
        }
    }
}

impl Iterator for GridScanner<'_> {
    type Item = Vec<Vec<char>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_col + 3 <= self.grid.cnt_columns() && self.curr_row + 3 <= self.grid.cnt_rows() {
            let first_row = self.grid[self.curr_row][self.curr_col..self.curr_col + 3].to_vec();
            let second_row =
                self.grid[self.curr_row + 1][self.curr_col..self.curr_col + 3].to_vec();
            let third_row = self.grid[self.curr_row + 2][self.curr_col..self.curr_col + 3].to_vec();

            let result = vec![first_row, second_row, third_row];

            self.curr_col += 1;

            if self.curr_col + 3 > self.grid.cnt_columns() {
                self.curr_row += 1;
                self.curr_col = 0;
            }

            Some(result)
        } else {
            None
        }
    }
}

impl Grid for Vec<Vec<char>> {
    fn cnt_rows(&self) -> usize {
        self.len()
    }
    fn cnt_columns(&self) -> usize {
        if let Some(row) = self.first() {
            row.len()
        } else {
            0
        }
    }
    fn rows(&self) -> &Vec<Vec<char>> {
        self
    }
    fn cols(&self) -> Vec<Vec<char>> {
        (0..self.cnt_columns())
            .into_iter()
            .map(|col_idx| {
                self.iter()
                    .map(|row| row[col_idx].clone())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    fn diagonals(&self) -> Vec<Vec<char>> {
        let top_forward_diagonals = (0..self.cnt_columns())
            .map(|col_idx| {
                (0..self.cnt_rows())
                    .flat_map(|row_idx| {
                        let new_col_idx = row_idx + col_idx;
                        if new_col_idx < self.cnt_columns() {
                            Some(self[row_idx][new_col_idx].clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let top_backward_diagonals = (0..self.cnt_columns())
            .into_iter()
            .rev()
            .map(|col_idx| {
                (0..self.cnt_rows())
                    .flat_map(|row_idx| {
                        let new_col_idx = col_idx.checked_sub(row_idx);
                        if let Some(new_col_idx) = new_col_idx {
                            Some(self[row_idx][new_col_idx].clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let side_forward_diagonals = (1..self.cnt_rows())
            .into_iter()
            .map(|row_idx| {
                (0..self.cnt_columns())
                    .into_iter()
                    .flat_map(|col_idx| {
                        let new_row_idx = row_idx + col_idx;
                        if col_idx < self.cnt_columns() && new_row_idx < self.cnt_rows() {
                            Some(self[new_row_idx][col_idx])
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let side_backward_diagonals = (1..self.cnt_rows())
            .into_iter()
            .map(|row_idx| {
                (0..self.cnt_columns())
                    .into_iter()
                    .rev()
                    .enumerate()
                    .flat_map(|(step, col_idx)| {
                        let new_row_idx = row_idx + step;
                        if col_idx < self.cnt_columns() && new_row_idx < self.cnt_rows() {
                            Some(self[new_row_idx][col_idx])
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        top_forward_diagonals
            .into_iter()
            .chain(top_backward_diagonals)
            .chain(side_forward_diagonals)
            .chain(side_backward_diagonals)
            .collect()
    }
}

fn build_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn first_part(input: &str) -> u64 {
    let grid = build_grid(input);
    const SEARCH_TERM: &str = "XMAS";
    let search_len = SEARCH_TERM.len();
    let count_row_hits: usize = grid
        .rows()
        .iter()
        .map(|row| {
            row.windows(search_len)
                .filter(|window| {
                    window.iter().collect::<String>() == SEARCH_TERM.to_string()
                        || window.iter().collect::<String>()
                            == SEARCH_TERM.chars().rev().collect::<String>()
                })
                .count()
        })
        .sum();
    let count_col_hits: usize = grid
        .cols()
        .iter()
        .map(|col| {
            col.windows(search_len)
                .filter(|window| {
                    window.iter().collect::<String>() == SEARCH_TERM.to_string()
                        || window.iter().collect::<String>()
                            == SEARCH_TERM.chars().rev().collect::<String>()
                })
                .count()
        })
        .sum();
    let diag_hits: usize = grid
        .diagonals()
        .iter()
        .filter(|diag| diag.len() >= SEARCH_TERM.len())
        .map(|diag| {
            diag.windows(search_len)
                .filter(|window| {
                    window.iter().collect::<String>() == SEARCH_TERM.to_string()
                        || window.iter().collect::<String>()
                            == SEARCH_TERM.chars().rev().collect::<String>()
                })
                .count()
        })
        .sum();

    (count_row_hits + count_col_hits + diag_hits) as u64
}

fn second_part(input: &str) -> u64 {
    let grid = build_grid(input);
    const SEARCH_TERM: &str = "MAS";

    let scanner = GridScanner::new(&grid);

    let result = scanner
        .into_iter()
        .filter(|window| {
            let diags = window.diagonals();
            diags.iter().filter(|diag| diag.len() == SEARCH_TERM.len()).all(|diag| {
                diag.iter().collect::<String>() == SEARCH_TERM.to_string()
                    || diag.iter().collect::<String>()
                        == SEARCH_TERM.chars().rev().collect::<String>()
            })
        })
        .count();

    result as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_first_part() {
        let input = include_str!("input_test");

        assert_eq!(first_part(input), 18);
    }

    #[test]
    fn test_grid() {
        let small_grid = "ABCDE\n12345\nDEFGH\n67890";

        let grid = build_grid(small_grid);

        let expected_rows = vec![
            ['A', 'B', 'C', 'D', 'E'],
            ['1', '2', '3', '4', '5'],
            ['D', 'E', 'F', 'G', 'H'],
            ['6', '7', '8', '9', '0'],
        ];

        assert_eq!(grid.rows(), &expected_rows);

        let expected_cols = vec![
            ['A', '1', 'D', '6'],
            ['B', '2', 'E', '7'],
            ['C', '3', 'F', '8'],
            ['D', '4', 'G', '9'],
            ['E', '5', 'H', '0'],
        ];

        assert_eq!(grid.cols(), expected_cols);

        let expected_diagonals = vec![
            vec!['A', '2', 'F', '9'],
            vec!['B', '3', 'G', '0'],
            vec!['C', '4', 'H'],
            vec!['D', '5'],
            vec!['E'],
            vec!['E', '4', 'F', '7'],
            vec!['D', '3', 'E', '6'],
            vec!['C', '2', 'D'],
            vec!['B', '1'],
            vec!['A'],
            vec!['1', 'E', '8'],
            vec!['D', '7'],
            vec!['6'],
            vec!['5', 'G', '8'],
            vec!['H', '9'],
            vec!['0'],
        ];

        assert_eq!(grid.diagonals(), expected_diagonals);
    }

    #[test]
    fn test_grid_scanner() {
        let grid = "ABCDE\nFGHIJ\nKLMNO\nPQRST";
        let grid = build_grid(grid);
        let mut scanner = GridScanner::new(&grid);

        let first = scanner.next().unwrap();
        let expected_first_rows = vec![
            vec!['A', 'B', 'C'],
            vec!['F', 'G', 'H'],
            vec!['K', 'L', 'M'],
        ];
        assert_eq!(first.rows(), &expected_first_rows);

        let second = scanner.next().unwrap();
        let expected_second_rows = vec![
            vec!['B', 'C', 'D'],
            vec!['G', 'H', 'I'],
            vec!['L', 'M', 'N'],
        ];
        assert_eq!(second.rows(), &expected_second_rows);

        let third = scanner.next().unwrap();
        let expected_third_rows = vec![
            vec!['C', 'D', 'E'],
            vec!['H', 'I', 'J'],
            vec!['M', 'N', 'O'],
        ];
        assert_eq!(third.rows(), &expected_third_rows);

        let fourth = scanner.next().unwrap();
        let expected_fourth_rows = vec![
            vec!['F', 'G', 'H'],
            vec!['K', 'L', 'M'],
            vec!['P', 'Q', 'R'],
        ];
        assert_eq!(fourth.rows(), &expected_fourth_rows);

        let fifth = scanner.next().unwrap();
        let expected_fifth_rows = vec![
            vec!['G', 'H', 'I'],
            vec!['L', 'M', 'N'],
            vec!['Q', 'R', 'S'],
        ];
        assert_eq!(fifth.rows(), &expected_fifth_rows);

        let sixth = scanner.next().unwrap();
        let expected_sixth_rows = vec![
            vec!['H', 'I', 'J'],
            vec!['M', 'N', 'O'],
            vec!['R', 'S', 'T'],
        ];
        assert_eq!(sixth.rows(), &expected_sixth_rows);
    }

    #[test]
    fn test_second_part() {
        let input = include_str!("input_test");
        
        assert_eq!(second_part(input), 9);
    }
}
