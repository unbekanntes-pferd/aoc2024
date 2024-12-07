fn main() {
    let input = include_str!("input");
    println!("First part: {}", first_part(input));
    //println!("Second part: {}", second_part(input));
}

#[derive(Clone, Copy, PartialEq)]
enum Block {
    Start,
    Unchecked,
    Checked,
    Blocked,
}

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

impl From<char> for Block {
    fn from(value: char) -> Self {
        match value {
            '#' => Block::Blocked,
            '^' => Block::Start,
            '.' => Block::Unchecked,
            _ => unimplemented!(),
        }
    }
}

struct Runner {
    curr_row: usize,
    curr_col: usize,
    curr_direction: Direction,
    grid: Vec<Vec<Block>>,
}

impl Runner {
    fn new(grid: Vec<Vec<Block>>) -> Self {
        let starts = grid.iter().enumerate().flat_map(|(row_idx, row)| {
            if let Some(col_idx) = row.iter().position(|block| block == &Block::Start) {
                Some((row_idx, col_idx))
            } else {
                None
            }
        }).collect::<Vec<_>>();
        let (start_row, start_col) = starts.first().unwrap();

        Self {
            curr_row: *start_row,
            curr_col: *start_col,
            curr_direction: Direction::Forward,
            grid
    }
}
}

impl Iterator for Runner {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let original_row = self.curr_row;
        let original_col = self.curr_col;


        if self.curr_row + 1 >= self.grid.len() || self.curr_col + 1 >= self.grid[0].len() || self.curr_row.checked_sub(1).is_none() || self.curr_col.checked_sub(1).is_none()  {
            return None
        }

        match self.curr_direction {
            Direction::Forward => {
                self.curr_row -= 1;
            }
            Direction::Backward => {
                self.curr_row += 1;
            }
            Direction::Left => {
                self.curr_col -= 1;
            }
            Direction::Right => {
                self.curr_col += 1;
            }
        }

        let next_block = self.grid[self.curr_row][self.curr_col];

        match next_block {
            Block::Blocked => {
                self.curr_row = original_row;
                self.curr_col = original_col;
                self.curr_direction = match self.curr_direction {
                    Direction::Forward => Direction::Right,
                    Direction::Right => Direction::Backward,
                    Direction::Backward => Direction::Left,
                    Direction::Left => Direction::Forward
                };

                Some(0)
            },
            Block::Unchecked => {
                self.grid[self.curr_row][self.curr_col] = Block::Checked;
                Some(1)
            },
            _ => Some(0)
        }
    }
}

fn build_grid(input: &str) -> Vec<Vec<Block>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| Block::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn first_part(input: &str) -> u64 {
    let grid = build_grid(input);
    let runner = Runner::new(grid);
    let visited: usize = runner.into_iter().sum();
    (visited + 1) as u64
}

fn second_part(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{build_grid, first_part, Runner};


    #[test]
    fn test_first_part() {
        let input = include_str!("input_test");
        let visited = first_part(input);
        assert_eq!(visited, 41);
    }
}
