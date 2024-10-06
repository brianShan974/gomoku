/// The chess piece in game of Gomoku. Can be either black or white.
#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Color {
    Black,
    White,
}
