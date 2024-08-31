use iced::widget::Text;

use crate::gui::{
    app::{AppCommand, AppElement},
    app_message::AppMessage,
    scene::{Scene, SceneUpdateResult},
};

pub const LAYOUT: (f32, f32, f32) = (0.375, 1.0, 0.375);

#[derive(Debug, Default)]
pub struct GameScene {}

impl Scene for GameScene {
    fn view(&self) -> AppElement<'_> {
        Text::new("").into()
    }

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult {
        SceneUpdateResult::CommandOnly(AppCommand::none())
    }
}
