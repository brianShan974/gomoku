use iced::Element;

use crate::{
    game_objects::playing::Move,
    gui::app::{AppCommand, AppElement, AppMessage},
};

pub const LAYOUT: (f32, f32, f32) = (0.375, 1.0, 0.375);

#[derive(Debug, Clone)]
pub enum GameMessage {
    Pause,
    Resume,
    Play(Move),
}

#[derive(Debug)]
pub struct GameScene {}

impl GameScene {
    pub fn view(&self) -> AppElement<'_> {
        unimplemented!()
    }

    pub fn update(&mut self, message: GameMessage) -> AppCommand {
        unimplemented!()
    }
}

impl From<GameMessage> for AppMessage {
    fn from(value: GameMessage) -> Self {
        Self::Game(value)
    }
}
