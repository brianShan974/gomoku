use piece::Color;

pub mod board;
pub mod piece;

pub mod playing;

/// The default sidelength of the board. A typical game has a 15 by 15 board.
pub const BOARD_SIDELENGTH: usize = 15;
/// The goal is to connect 5 pieces in a row, so the goal is 5.
pub(super) const WIN_GOAL: usize = 5;
/// There are 4 directions in total around one position that need to be checked to determine if there is a winner.
/// Two of them are vertical and horizontal, and the other 2 are diagonal.
pub(super) const DIRECTIONS: usize = 4;
/// To determine if there is a winner, 4 crosses to each direction need to be checked so the max
/// offset is 4.
pub(super) const MAX_OFFSET: usize = 4;

pub type ChessPiece = Color;
pub type Player = Color;

pub(crate) type Cross = Option<ChessPiece>;
pub(crate) type Winner = Player;
pub(crate) type Pos = (usize, usize);

type FivePos = Option<[Pos; WIN_GOAL]>;
type FivePosSeq = [FivePos; WIN_GOAL];
