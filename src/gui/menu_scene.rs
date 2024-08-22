use iced::Command;

use crate::gui::app::{AppCommand, AppElement, AppMessage};

#[derive(Debug, Clone)]
pub enum MenuMessage {}

#[derive(Debug)]
pub struct MenuScene {}

impl MenuScene {
    pub fn view(&self) -> AppElement<'_> {
        unimplemented!()
    }

    pub fn update(&mut self, message: MenuMessage) -> AppCommand {
        unimplemented!()
    }
}

impl From<MenuMessage> for AppMessage {
    fn from(value: MenuMessage) -> Self {
        Self::Menu(value)
    }
}
