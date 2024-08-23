use crate::gui::{
    app::{AppCommand, AppElement},
    game::message::GameMessage,
};

pub const LAYOUT: (f32, f32, f32) = (0.375, 1.0, 0.375);

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
