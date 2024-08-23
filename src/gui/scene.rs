use crate::gui::app::{AppCommand, AppElement, AppMessage};

#[derive(Debug, Default)]
pub enum CurrentScene {
    Start,
    Game,
    Paused,
    #[default]
    RoleSelection,
}

pub trait Scene {
    fn view(&self) -> AppElement<'_>;

    fn update(&mut self, message: AppMessage) -> AppCommand;
}
