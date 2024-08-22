use iced::Command;

use crate::gui::app::{AppCommand, ElementType, GUIMessage};

#[derive(Debug, Clone)]
pub enum MenuMessage {}

#[derive(Debug)]
pub struct MenuScene {}

impl MenuScene {
    pub fn view(&self) -> ElementType<'_> {
        unimplemented!()
    }

    pub fn update(&mut self, message: MenuMessage) -> AppCommand {
        unimplemented!()
    }
}

impl From<MenuMessage> for GUIMessage {
    fn from(value: MenuMessage) -> Self {
        Self::Menu(value)
    }
}
