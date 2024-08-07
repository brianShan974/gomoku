/// The chess piece in game of Gomoku. Can be either black or white.
#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) enum Color {
    Black,
    White,
}

pub(super) type Player = Color;
pub(super) type ChessPiece = Color;
