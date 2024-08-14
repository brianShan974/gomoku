use super::{board::Pos, piece::Player};

#[derive(Debug, Clone)]
pub(crate) struct Move {
    player: Player,
    pos: Pos,
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

pub(super) enum PlayingError {
    PositionOccupied,
}
