use crate::gui::{
    app::AppElement,
    app_message::AppMessage,
    scene::{Scene, UpdateResult},
};

pub const LAYOUT: (f32, f32, f32) = (0.375, 1.0, 0.375);

#[derive(Debug)]
pub struct GameScene {}

impl Scene for GameScene {
    fn view(&self) -> AppElement<'_> {
        unimplemented!()
    }

    fn update(&mut self, message: AppMessage) -> UpdateResult {
        unimplemented!()
    }
}
