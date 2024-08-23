use crate::{game_objects::playing::Move, gui::app_message::AppMessage};

#[derive(Debug, Clone)]
pub enum GameMessage {
    Pause,
    Resume,
    Play(Move),
}

impl From<GameMessage> for AppMessage {
    fn from(value: GameMessage) -> Self {
        Self::Game(value)
    }
}
