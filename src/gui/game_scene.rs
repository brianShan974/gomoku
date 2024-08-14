use iced::Element;

use crate::{game_objects::playing::Move, gui::app::ElementType};

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
}
