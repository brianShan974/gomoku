use crate::gui::{
    connecting::message::ConnectingMessage, game::message::GameMessage, menu::message::MenuMessage,
    role_selection::message::RoleSelectionMessage,
};

#[derive(Debug, Clone)]
pub enum AppMessage {
    SelectRole(RoleSelectionMessage),
    Connecting(ConnectingMessage),
    Game(GameMessage),
    Menu(MenuMessage),
    // Nothing,
    Exit,
}
