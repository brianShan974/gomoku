use crate::gui::{
    connecting_scenes::connecting_scene::ConnectingScene, game_scene::GameScene,
    menu_scene::MenuScene, role_selection_scene::RoleSelectionScene,
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
