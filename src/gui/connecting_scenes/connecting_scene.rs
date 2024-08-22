use iced::{widget::text_editor::Action, Command};

use crate::gui::{
    app::{AppCommand, AppMessage},
    connecting_scenes::{
        client_connecting_scene::ClientConnectingScene,
        server_connecting_scene::ServerConnectingScene,
    },
};

#[derive(Debug)]
pub enum ConnectingScene {
    Client(ClientConnectingScene),
    Server(ServerConnectingScene),
}

#[derive(Debug, Clone)]
pub enum ConnectingMessage {
    Edit(Action),
    Connect,
    Return,
}

impl Default for ConnectingScene {
    fn default() -> Self {
        Self::Client(ClientConnectingScene::default())
    }
}

impl ConnectingScene {
    pub fn update(&mut self, message: ConnectingMessage) -> AppCommand {
        match self {
            Self::Client(client_starting_scene) => {
                let _ = client_starting_scene.update(message);
            }
            Self::Server(server_starting_scene) => {
                let _ = server_starting_scene.update(message);
            }
        }
        Command::none()
    }
}

impl From<ConnectingMessage> for AppMessage {
    fn from(value: ConnectingMessage) -> Self {
        Self::Connecting(value)
    }
}
