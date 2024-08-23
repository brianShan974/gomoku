use crate::gui::{
    app::{AppCommand, AppMessage},
    connecting::{client_scene::ClientConnectingScene, server_scene::ServerConnectingScene},
    scene::Scene,
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

impl Scene for ConnectingScene {
    fn view(&self) -> crate::gui::app::AppElement<'_> {
        match self {
            Self::Client(scene) => scene.view(),
            Self::Server(scene) => scene.view(),
        }
    }

    fn update(&mut self, message: AppMessage) -> AppCommand {
        match self {
            Self::Client(scene) => scene.update(message),
            Self::Server(scene) => scene.update(message),
        }
    }
}
