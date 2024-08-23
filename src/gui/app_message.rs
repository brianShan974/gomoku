use crate::gui::{
    connecting::message::ConnectingMessage, game::message::GameMessage, menu::scene::MenuMessage,
    role_selection::scene::RoleSelectionMessage,
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
