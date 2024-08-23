use iced::Command;

use crate::gui::{
    app::{AppCommand, AppMessage},
    connecting::{
        client_scene::ClientConnectingScene, message::ConnectingMessage,
        server_scene::ServerConnectingScene,
    },
};

#[derive(Debug)]
pub enum ConnectingScene {
    Client(ClientConnectingScene),
    Server(ServerConnectingScene),
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
