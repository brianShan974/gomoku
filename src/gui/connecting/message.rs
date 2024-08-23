use iced::widget::text_editor::Action;

use crate::gui::app_message::AppMessage;

#[derive(Debug, Clone)]
pub enum ConnectingMessage {
    Edit(Action),
    Connect,
    Return,
}

impl From<ConnectingMessage> for AppMessage {
    fn from(value: ConnectingMessage) -> Self {
        Self::Connecting(value)
    }
}
