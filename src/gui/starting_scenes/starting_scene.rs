use crate::gui::starting_scenes::{
    client_starting_scene::ClientStartingScene, server_starting_scene::ServerStartingScene,
};

#[derive(Debug)]
pub enum StartingScene {
    Client(ClientStartingScene),
    Server(ServerStartingScene),
}

impl Default for StartingScene {
    fn default() -> Self {
        Self::Client(ClientStartingScene::default())
    }
}
