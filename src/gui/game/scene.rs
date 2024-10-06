use iced::widget::{
    canvas,
    canvas::{Fill, Program},
    container, Text,
};

use crate::{
    game_objects::board::Board,
    gui::{
        app::{AppCommand, AppElement},
        app_message::AppMessage,
        scene::{Scene, SceneUpdateResult},
    },
};

use super::geometry::gameboard::GameBoard;

pub const LAYOUT: (f32, f32, f32) = (0.375, 1.0, 0.375);

#[derive(Debug, Default)]
pub struct GameScene {
    game_board: Board,
}

impl Scene for GameScene {
    fn view(&self) -> AppElement<'_> {
        canvas(GameBoard::default()).into()
    }

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult {
        SceneUpdateResult::CommandOnly(AppCommand::none())
    }
}
