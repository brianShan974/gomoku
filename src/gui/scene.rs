use crate::gui::{
    app::{AppCommand, AppElement},
    app_message::AppMessage,
    role_selection::scene::Role,
};

#[derive(Debug, Default)]
pub enum SceneType {
    Connecting(Role),
    Game,
    Menu,
    #[default]
    RoleSelection,
}

pub enum SceneUpdateResult {
    SceneSwitch(SceneType, Box<dyn Scene>, AppCommand),
    CommandOnly(AppCommand),
}

pub trait Scene {
    fn view(&self) -> AppElement<'_>;

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult;
}
