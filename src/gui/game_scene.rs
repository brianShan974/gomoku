use iced::Element;

use crate::{
    game_objects::playing::Move,
    gui::app::{AppCommand, ElementType, GUIMessage},
};

pub const LAYOUT: (f32, f32, f32) = (0.375, 1.0, 0.375);

#[derive(Debug, Clone)]
pub enum GomokuMessage {
    Pause,
    Resume,
    Play(Move),
}

#[derive(Debug)]
pub struct GameScene {}

impl GameScene {
    pub fn view(&self) -> ElementType<'_> {
        unimplemented!()
    }

    pub fn update(&mut self, message: GomokuMessage) -> AppCommand {
        unimplemented!()
    }
}

impl From<GomokuMessage> for GUIMessage {
    fn from(value: GomokuMessage) -> Self {
        Self::Game(value)
    }
}
