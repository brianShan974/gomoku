use iced::Command;

use crate::gui::{app::ElementType, starting_scenes::starting_scene::StartingMessage};

#[derive(Debug, Default)]
pub struct ServerStartingScene;

impl ServerStartingScene {
    pub fn view(&self) -> ElementType<'_> {
        unimplemented!()
    }

    pub fn update(&mut self, message: StartingMessage) -> Command<StartingMessage> {
        unimplemented!()
    }
}
