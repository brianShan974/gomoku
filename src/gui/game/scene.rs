use crate::gui::{
    app::{AppCommand, AppElement, AppMessage},
    scene::Scene,
};

pub const LAYOUT: (f32, f32, f32) = (0.375, 1.0, 0.375);

#[derive(Debug)]
pub struct GameScene {}

impl Scene for GameScene {
    fn view(&self) -> AppElement<'_> {
        unimplemented!()
    }

    fn update(&mut self, message: AppMessage) -> AppCommand {
        unimplemented!()
    }
}
