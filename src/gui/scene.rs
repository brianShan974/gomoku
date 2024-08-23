use crate::gui::{
    app::{AppCommand, AppElement},
    app_message::AppMessage,
    role_selection::scene::Role,
};

#[derive(Debug, Default)]
pub enum CurrentScene {
    Connecting(Role),
    Game,
    Menu,
    #[default]
    RoleSelection,
}

pub trait Scene {
    fn view(&self) -> AppElement<'_>;

    fn update(&mut self, message: AppMessage) -> AppCommand;
}
