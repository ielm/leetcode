fn _backtrack<T>(
    path: &mut Vec<T>,
    result: &mut Vec<Vec<T>>,
    choices: &[T],
    is_valid_choice: &dyn Fn(&Vec<T>, &T) -> bool,
) -> i32
where
    T: Clone,
{
    if is_goal(path) {
        result.push(path.clone());
        return 1;
    }

    let mut count = 0;
    for choice in choices.iter() {
        if !is_valid_choice(path, choice) {
            continue;
        }
        path.push(choice.clone());
        count += _backtrack(path, result, choices, is_valid_choice);
        path.pop();
    }
    count
}

fn is_goal<T>(_path: &[T]) -> bool {
    // Implement the logic to check if the current path is a goal
    // For example, if the goal is to reach a certain length:
    // path.len() == desired_length
    unimplemented!()
}

// fn backtrack<T>(curr: T) -> i32 {
//     // IF BASE CASE
//     //     return 0
//     if

// }

struct Solution {}

/// Represents a Sudoku puzzle with methods to solve it.
struct Sudoku<'a> {
    /// A mutable reference to a 2D vector representing the Sudoku board.
    board: &'a mut Vec<Vec<char>>,
    /// Tracking the count of digits in each 3x3 square.
    squares: [[usize; 10]; 9],
    /// Tracking the count of digits in each row.
    rows: [[usize; 10]; 9],
    /// Tracking the count of digits in each column.
    cols: [[usize; 10]; 9],
}

impl<'a> Sudoku<'a> {
    /// Constructs a new Sudoku instance and initializes tracking arrays.
    pub fn new(board: &'a mut Vec<Vec<char>>) -> Self {
        let mut sudoku = Self {
            board,
            squares: [[0usize; 10]; 9],
            rows: [[0usize; 10]; 9],
            cols: [[0usize; 10]; 9],
        };

        // Pre-fill the tracking arrays with the existing digits on the board.
        for row in 0..9 {
            for col in 0..9 {
                if sudoku.board[row][col] != '.' {
                    sudoku.place(row, col, sudoku.board[row][col]);
                }
            }
        }
        sudoku
    }

    /// Attempts to solve the Sudoku puzzle starting from the given index.
    pub fn solve(&mut self, i: usize) -> bool {
        // If we have reached the end of the board, the puzzle is solved.
        if i >= 81 {
            return true;
        }
        let (row, col) = (i / 9, i % 9);

        // If the current cell is already filled, move to the next cell.
        if self.board[row][col] != '.' {
            return self.solve(i + 1);
        }

        // Try each digit from 1 to 9 in the current cell.
        for digit in '1'..='9' {
            if self.is_valid(row, col, digit) {
                // Place the digit and continue solving the rest of the board.
                self.place(row, col, digit);
                if self.solve(i + 1) {
                    return true;
                }
                // If placing the digit didn't lead to a solution, remove it.
                self.reverse(row, col);
            }
        }

        // If no digit leads to a solution, the puzzle cannot be solved from this state.
        false
    }

    /// Checks if placing a digit at the given row and column is valid according to Sudoku rules.
    pub fn is_valid(&self, row: usize, col: usize, digit: char) -> bool {
        let digit = digit as usize - '0' as usize;

        // Check if the digit is not already present in the corresponding row, column, and square.
        self.squares[row / 3 * 3 + col / 3][digit] == 0
            && self.rows[row][digit] == 0
            && self.cols[col][digit] == 0
    }

    /// Places a digit at the given row and column and updates tracking arrays.
    pub fn place(&mut self, row: usize, col: usize, digit: char) {
        self.board[row][col] = digit;

        let digit = digit as usize - '0' as usize;

        // Increment the count of the digit in the corresponding row, column, and square.
        self.squares[row / 3 * 3 + col / 3][digit] += 1;
        self.rows[row][digit] += 1;
        self.cols[col][digit] += 1;
    }

    /// Removes a digit from the given row and column and updates tracking arrays.
    pub fn reverse(&mut self, row: usize, col: usize) {
        let digit = self.board[row][col] as usize - '0' as usize;

        // Decrement the count of the digit in the corresponding row, column, and square.
        self.squares[row / 3 * 3 + col / 3][digit] -= 1;
        self.rows[row][digit] -= 1;
        self.cols[col][digit] -= 1;

        // Reset the cell to be empty.
        self.board[row][col] = '.';
    }
}

/// Static methods for solving Sudoku puzzles.
impl Solution {
    /// Solves the given Sudoku puzzle in-place.
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut sudoku = Sudoku::new(board);
        sudoku.solve(0);
    }
}
