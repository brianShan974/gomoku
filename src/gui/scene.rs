use crate::gui::{
    connecting::scene::ConnectingScene, game::scene::GameScene, menu::scene::MenuScene,
    role_selection::scene::RoleSelectionScene,
};

#[derive(Debug)]
pub enum Scene {
    Start(ConnectingScene),
    Game(GameScene),
    Paused(MenuScene),
    RoleSelection(RoleSelectionScene),
}

impl Default for Scene {
    fn default() -> Self {
        Self::RoleSelection(RoleSelectionScene)
    }
}
