use crate::game_objects::piece::Color;
use crate::game_objects::playing::{Move, PlayingError};

use super::playing::UndoError;
use super::{
    ChessPiece, Cross, FivePosSeq, Player, Pos, Winner, BOARD_SIDELENGTH, DIRECTIONS, MAX_OFFSET,
    WIN_GOAL,
};

/// The board struct.
#[derive(Debug, Default)]
pub struct Board {
    pub(super) grid: [[Cross; BOARD_SIDELENGTH]; BOARD_SIDELENGTH],
    moves: Vec<Move>,
}

impl Board {
    pub fn apply_move(&mut self, next_move: Move) -> Result<(), PlayingError> {
        let pos = next_move.get_pos();
        if self.get_on_pos(pos).is_some() {
            return Err(PlayingError::PositionOccupied);
        }

        let chess_piece = next_move.get_player();
        self.set_on_pos(pos, chess_piece);
        self.moves.push(next_move);

        Ok(())
    }

    pub fn undo_move(&mut self, current_player: Player) -> Result<(), UndoError> {
        if self.moves.is_empty() {
            Err(UndoError::NoMorePieceOnBoard)
        } else if !self.player_can_undo(current_player) {
            Err(UndoError::CurrentPlayerCannotUndo)
        } else {
            let last_move = self.moves.pop().unwrap();
            self.remove_on_pos(last_move.get_pos());

            Ok(())
        }
    }

    /// Get the piece on a given position on the grid.
    fn get_on_pos(&self, pos: Pos) -> Cross {
        let (row, col) = pos;
        if row >= BOARD_SIDELENGTH || col >= BOARD_SIDELENGTH {
            return None;
        }

        self.grid[row][col]
    }

    fn set_on_pos(&mut self, pos: Pos, piece: ChessPiece) {
        let (row, col) = pos;
        self.grid[row][col] = Some(piece);
    }

    fn player_can_undo(&self, player: Player) -> bool {
        self.moves.last().unwrap().get_player() == player.get_opponent()
    }

    fn remove_on_pos(&mut self, pos: Pos) {
        let (row, col) = pos;
        self.grid[row][col] = None;
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
    let col_ub = if col <= BOARD_SIDELENGTH - WIN_GOAL {
        col
    } else {
        BOARD_SIDELENGTH - WIN_GOAL
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
    let row_ub = if row <= BOARD_SIDELENGTH - WIN_GOAL {
        row
    } else {
        BOARD_SIDELENGTH - WIN_GOAL
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
        if col > BOARD_SIDELENGTH - WIN_GOAL {
            row + ((BOARD_SIDELENGTH - 1) - col)
        } else {
            row
        }
    } else if row > BOARD_SIDELENGTH - WIN_GOAL {
        BOARD_SIDELENGTH - WIN_GOAL
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

    let row_lb = if row + col >= BOARD_SIDELENGTH - 1 {
        if col > BOARD_SIDELENGTH - WIN_GOAL {
            row - ((BOARD_SIDELENGTH - 1) - col)
        } else {
            row - MAX_OFFSET
        }
    } else if row < WIN_GOAL {
        0
    } else {
        row - MAX_OFFSET
    };
    let row_ub = if row + col <= BOARD_SIDELENGTH - 1 {
        if col < WIN_GOAL {
            row + col
        } else {
            row
        }
    } else if row > BOARD_SIDELENGTH - WIN_GOAL {
        BOARD_SIDELENGTH - WIN_GOAL
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
