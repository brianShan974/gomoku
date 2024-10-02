use crate::gui::{
    game::message::GameMessage, menu::message::MenuMessage, network::message::ConnectingMessage,
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
