use iced::{widget::text_editor::Action, Command};

use crate::gui::{
    app::{AppCommand, GUIMessage},
    starting_scenes::{
        client_starting_scene::{self, ClientStartingScene},
        server_starting_scene::ServerStartingScene,
    },
};

#[derive(Debug)]
pub enum StartingScene {
    Client(ClientStartingScene),
    Server(ServerStartingScene),
}

#[derive(Debug, Clone)]
pub enum StartingMessage {
    Edit(Action),
    Connect,
    Return,
}

impl Default for StartingScene {
    fn default() -> Self {
        Self::Client(ClientStartingScene::default())
    }
}

impl StartingScene {
    pub fn update(&mut self, message: StartingMessage) -> AppCommand {
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

impl From<StartingMessage> for GUIMessage {
    fn from(value: StartingMessage) -> Self {
        Self::Start(value)
    }
}
