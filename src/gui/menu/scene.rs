use iced::widget::{button, row};

use crate::gui::{
    app::{AppCommand, AppElement, AppMessage},
    scene::Scene,
};

#[derive(Debug, Clone)]
pub enum MenuMessage {
    Resign,
    Undo,
}

#[derive(Debug)]
pub struct MenuScene {}

impl Scene for MenuScene {
    fn view(&self) -> AppElement<'_> {
        let resign_button = button("RESIGN").on_press(MenuMessage::Resign.into());
        let undo_button = button("UNDO").on_press(MenuMessage::Undo.into());

        row!(resign_button, undo_button).into()
    }

    fn update(&mut self, message: AppMessage) -> AppCommand {
        unimplemented!()
    }
}

impl From<MenuMessage> for AppMessage {
    fn from(value: MenuMessage) -> Self {
        Self::Menu(value)
    }
}