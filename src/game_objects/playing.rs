use super::{Player, Pos};

#[derive(Debug, Clone)]
pub(crate) struct Move {
    player: Player,
    pos: Pos,
}

pub enum PlayingError {
    PositionOccupied,
}

pub enum UndoError {
    NoMorePieceOnBoard,
    CurrentPlayerCannotUndo,
}

impl Move {
    pub fn new(player: Player, pos: Pos) -> Self {
        Self { player, pos }
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }

    pub fn get_player(&self) -> Player {
        self.player
    }
}

impl Player {
    pub fn get_opponent(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}
