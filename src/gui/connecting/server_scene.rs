use iced::Command;

use crate::gui::{
    app::{AppCommand, AppElement, AppMessage},
    connecting::message::ConnectingMessage,
    scene::Scene,
};

#[derive(Debug, Default)]
pub struct ServerConnectingScene;

impl Scene for ServerConnectingScene {
    fn view(&self) -> AppElement<'_> {
        unimplemented!()
    }

    fn update(&mut self, message: AppMessage) -> AppCommand {
        unimplemented!()
    }
}
