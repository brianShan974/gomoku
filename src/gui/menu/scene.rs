use iced::{
    widget::{button, row},
    Command,
};

use crate::gui::app::{AppCommand, AppElement, AppMessage};

#[derive(Debug, Clone)]
pub enum MenuMessage {
    Resign,
    Undo,
}

#[derive(Debug)]
pub struct MenuScene {}

impl MenuScene {
    pub fn view(&self) -> AppElement<'_> {
        let resign_button = button("RESIGN").on_press(MenuMessage::Resign.into());
        let undo_button = button("UNDO").on_press(MenuMessage::Undo.into());

        row!(resign_button, undo_button).into()
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
