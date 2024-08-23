use iced::Command;

use crate::gui::{app::AppElement, connecting::message::ConnectingMessage};

#[derive(Debug, Default)]
pub struct ServerConnectingScene;

impl ServerConnectingScene {
    pub fn view(&self) -> AppElement<'_> {
        unimplemented!()
    }

    pub fn update(&mut self, message: ConnectingMessage) -> Command<ConnectingMessage> {
        unimplemented!()
    }
}
