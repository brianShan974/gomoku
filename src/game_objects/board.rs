use sdl2::libc::abs;

use crate::game_objects::piece::Piece;

/// The default size of the board. A typical game has a 15 by 15 board.
const BOARD_SIZE: usize = 15;
/// The goal is to connect 5 pieces in a row, so the goal is 5.
const WIN_GOAL: usize = 5;

type Cross = Option<Piece>;
type Winner = Piece;
type Row = [Cross; BOARD_SIZE];
type Col = [Cross; BOARD_SIZE];
type MainDiag = Vec<Cross>;
type OffDiag = Vec<Cross>;
type Rows = [Row; BOARD_SIZE];
type Cols = [Col; BOARD_SIZE];
type MainDiags = [MainDiag; (BOARD_SIZE - WIN_GOAL) * 2 - 1];
type OffDiags = [OffDiag; (BOARD_SIZE - WIN_GOAL) * 2 - 1];

/// The board struct.
#[derive(Debug, Default)]
pub struct Board {
    grid: [[Cross; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Construct a board from a given grid.
    pub fn new(grid: [[Cross; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        Self { grid }
    }

    /// Try to place the piece on a position. Fails if the position is occupied.
    pub fn place_on_pos(&mut self, pos: (usize, usize), piece: Piece) -> Result<(), String> {
        match self.grid[pos.0][pos.1] {
            None => {
                self.grid[pos.0][pos.1] = Some(piece);
                Ok(())
            }
            Some(_) => Err(String::from("The position is already occupied!")),
        }
    }

    /// Get the piece on a given position on the grid.
    pub fn get_on_pos(&self, pos: (usize, usize)) -> Cross {
        self.grid[pos.0][pos.1]
    }

    /// Try to tell if there is a winner. Return None if the game is not over.
    pub fn determine_winnner(&self) -> Option<Winner> {
        unimplemented!()
    }

    /// Get a row from the board.
    fn get_row(&self, row: usize) -> Row {
        self.grid[row]
    }

    /// Get a column from the board.
    fn get_col(&self, col: usize) -> Col {
        let mut column: Col = [None; BOARD_SIZE];
        for (i, row) in self.grid.iter().enumerate() {
            column[i] = row[col];
        }
        column
    }

    /// Get a main diagonal from the board.
    fn get_main_diag(&self, diag: isize) -> MainDiag {
        if diag >= 0 {
            let count = BOARD_SIZE - diag as usize;
            Vec::new()
        } else {
            Vec::new()
        }
    }

    /// Get an off diagonal from the board.
    fn get_off_diag(&self, diag: isize) -> OffDiag {
        unimplemented!()
    }

    /// Get all rows from the board.
    fn get_rows(&self) -> Rows {
        unimplemented!()
    }

    /// Get all columns from the board.
    fn get_cols(&self) -> Cols {
        unimplemented!()
    }

    /// Get all main diagonals from the board.
    fn get_main_diags(&self) -> MainDiags {
        unimplemented!()
    }

    /// Get all off diagonals from the board.
    fn get_off_diags(&self) -> OffDiags {
        unimplemented!()
    }

    /// Check a row to see if there is a winner.
    fn check_row(&self, row: usize) -> Option<Winner> {
        let row = self.get_row(row);
        check_five_equal_in_array(&row)
    }

    /// Check a column to see if there is a winner.
    fn check_col(&self, col: usize) -> Option<Winner> {
        let col = self.get_col(col);
        check_five_equal_in_array(&col)
    }

    /// Check a main diagonal to see if there is a winner.
    fn check_main_diag(&self, diag: isize) -> Option<Winner> {
        let diag = self.get_main_diag(diag);
        check_five_equal_in_array(&diag)
    }

    /// Check an off diagonal to see if there is a winner.
    fn check_off_diag(&self, diag: isize) -> Option<Winner> {
        let diag = self.get_off_diag(diag);
        check_five_equal_in_array(&diag)
    }

    /// Check all rows to see if there is a winner.
    fn check_rows(&self) -> Option<Winner> {
        unimplemented!()
    }
}

fn check_five_equal<T>(a: T, b: T, c: T, d: T, e: T) -> bool
where
    T: PartialEq,
{
    a == b && b == c && c == d && d == e
}

fn check_five_equal_in_array(array: &[Cross]) -> Option<Winner> {
    let array_len = array.len();
    if array_len < WIN_GOAL {
        return None;
    }

    for starting_index in 0..(array_len - WIN_GOAL) {
        if array[starting_index].is_some()
            && check_five_equal(
                array[starting_index],
                array[starting_index + 1],
                array[starting_index + 2],
                array[starting_index + 3],
                array[starting_index + 4],
            )
        {
            return array[starting_index];
        }
    }
    None
}
