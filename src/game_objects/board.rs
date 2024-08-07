use crate::game_objects::piece::{ChessPiece, Player};

use super::piece::Color;
use super::playing::{Move, PlayingError};

/// The default size of the board. A typical game has a 15 by 15 board.
pub(super) const BOARD_SIZE: usize = 15;
/// The goal is to connect 5 pieces in a row, so the goal is 5.
pub(super) const WIN_GOAL: usize = 5;
/// There are 4 directions in total around one position that need to be checked to determine if there is a winner.
/// Two of them are vertical and horizontal, and the other 2 are diagonal.
pub(super) const DIRECTIONS: usize = 4;
/// To determine if there is a winner, 4 crosses to each direction need to be checked so the max
/// offset is 4.
pub(super) const MAX_OFFSET: usize = 4;

pub(super) type Cross = Option<ChessPiece>;
pub(super) type Winner = Player;
pub(super) type Pos = (usize, usize);

type FivePos = Option<[Pos; WIN_GOAL]>;
type FivePosSeq = [FivePos; WIN_GOAL];

/// The board struct.
#[derive(Debug, Default)]
pub struct Board {
    pub(super) grid: [[Cross; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Construct a board from a given grid.
    pub fn new(grid: [[Cross; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        Self { grid }
    }

    /// Try to place the piece on a position. Fails if the position is occupied.
    pub fn place_on_pos(&mut self, pos: (usize, usize), piece: Color) -> Result<(), String> {
        match self.grid[pos.0][pos.1] {
            None => {
                self.grid[pos.0][pos.1] = Some(piece);
                Ok(())
            }
            Some(_) => Err(String::from("The position is already occupied!")),
        }
    }

    /// Get the piece on a given position on the grid.
    pub fn get_on_pos(&self, pos: Pos) -> Cross {
        let (row, col) = pos;
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return None;
        }

        self.grid[row][col]
    }

    pub fn set_on_pos(&mut self, pos: Pos, piece: ChessPiece) {
        let (row, col) = pos;
        self.grid[row][col] = Some(piece);
    }

    fn apply_move(&mut self, next_move: Move) -> Result<(), PlayingError> {
        let pos = next_move.get_pos();
        if self.get_on_pos(pos).is_some() {
            return Err(PlayingError::PositionOccupied);
        }

        let chess_piece = next_move.get_player();
        self.set_on_pos(pos, chess_piece);

        Ok(())
    }

    /// Try to tell if there is a winner. Return None if the game is not over.
    fn determine_winner(&self, pos: Pos) -> Option<Winner> {
        let possible_winner: Winner = self.get_on_pos(pos).unwrap();
        let (row, col) = pos;

        let directions = get_five_around(row, col);

        for direction in directions {
            for five_pos in direction {
                if five_pos.unwrap().iter().all(|&pos| {
                    if let Some(cross) = self.get_on_pos(pos) {
                        cross == possible_winner
                    } else {
                        false
                    }
                }) {
                    return Some(possible_winner);
                }
            }
        }

        None
    }
}

fn get_five_around(row: usize, col: usize) -> [FivePosSeq; DIRECTIONS] {
    [
        get_horizontal_fives(row, col),
        get_vertical_fives(row, col),
        get_main_diag_fives(row, col),
        get_off_diag_fives(row, col),
    ]
}

fn get_horizontal_fives(row: usize, col: usize) -> FivePosSeq {
    let mut result = [None; WIN_GOAL];

    // column lower bound
    let col_lb = if col < WIN_GOAL { 0 } else { col - MAX_OFFSET };
    let col_ub = if col <= BOARD_SIZE - WIN_GOAL {
        col
    } else {
        BOARD_SIZE - WIN_GOAL
    };

    for (i, col) in (col_lb..=col_ub).enumerate() {
        result[i] = Some([
            (row, col),
            (row, col + 1),
            (row, col + 2),
            (row, col + 3),
            (row, col + 4),
        ]);
    }

    result
}

fn get_vertical_fives(row: usize, col: usize) -> FivePosSeq {
    let mut result = [None; WIN_GOAL];

    // row lower bound
    let row_lb = if row < WIN_GOAL { 0 } else { row - MAX_OFFSET };
    let row_ub = if row <= BOARD_SIZE - WIN_GOAL {
        row
    } else {
        BOARD_SIZE - WIN_GOAL
    };

    for (i, row) in (row_lb..=row_ub).enumerate() {
        result[i] = Some([
            (row, col),
            (row + 1, col),
            (row + 2, col),
            (row + 3, col),
            (row + 4, col),
        ]);
    }

    result
}

fn get_main_diag_fives(row: usize, col: usize) -> FivePosSeq {
    let mut result = [None; WIN_GOAL];

    let row_lb = if row >= col {
        if col < WIN_GOAL {
            row - col
        } else {
            row - MAX_OFFSET
        }
    } else if row < WIN_GOAL {
        0
    } else {
        row - MAX_OFFSET
    };
    let row_ub = if row <= col {
        if col > BOARD_SIZE - WIN_GOAL {
            row + ((BOARD_SIZE - 1) - col)
        } else {
            row
        }
    } else if row > BOARD_SIZE - WIN_GOAL {
        BOARD_SIZE - WIN_GOAL
    } else {
        row
    };
    let col_lb = col - (row - row_lb);

    for (i, row) in (row_lb..=row_ub).enumerate() {
        result[i] = Some([
            (row, col_lb + i),
            (row + 1, col_lb + i + 1),
            (row + 2, col_lb + i + 2),
            (row + 3, col_lb + i + 3),
            (row + 4, col_lb + i + 4),
        ]);
    }

    result
}

fn get_off_diag_fives(row: usize, col: usize) -> FivePosSeq {
    let mut result = [None; WIN_GOAL];

    let row_lb = if row + col >= BOARD_SIZE - 1 {
        if col > BOARD_SIZE - WIN_GOAL {
            row - ((BOARD_SIZE - 1) - col)
        } else {
            row - MAX_OFFSET
        }
    } else if row < WIN_GOAL {
        0
    } else {
        row - MAX_OFFSET
    };
    let row_ub = if row + col <= BOARD_SIZE - 1 {
        if col < WIN_GOAL {
            row + col
        } else {
            row
        }
    } else if row > BOARD_SIZE - WIN_GOAL {
        BOARD_SIZE - WIN_GOAL
    } else {
        row
    };
    let col_ub = col + (row - row_lb);

    for (i, row) in (row_lb..=row_ub).enumerate() {
        result[i] = Some([
            (row, col_ub - i),
            (row + 1, col_ub - i + 1),
            (row + 2, col_ub - i + 2),
            (row + 3, col_ub - i + 3),
            (row + 4, col_ub - i + 4),
        ]);
    }

    result
}
