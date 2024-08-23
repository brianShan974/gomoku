use iced::{
    widget::{button, column, row, Text},
    Command,
};

use crate::gui::app::{AppCommand, AppElement, AppMessage};

#[derive(Debug, Default)]
pub struct RoleSelectionScene;

#[derive(Debug, Clone)]
pub enum RoleSelectionMessage {
    ChooseClient,
    ChooseServer,
}

#[derive(Debug, Clone, Default)]
pub enum Role {
    #[default]
    Client,
    Server,
}

impl RoleSelectionScene {
    pub fn view(&self) -> AppElement<'_> {
        let user_hint = Text::new("Please select whether you want to be the client or the server:");

        let server_button = button("SERVER").on_press(RoleSelectionMessage::ChooseServer.into());
        let client_button = button("CLIENT").on_press(RoleSelectionMessage::ChooseClient.into());
        let buttons = row!(server_button, client_button);

        let exit_button = button("EXIT").on_press(AppMessage::Exit);

        column!(user_hint, buttons, exit_button).into()
    }

    pub fn update(&mut self, message: RoleSelectionMessage) -> AppCommand {
        unimplemented!()
    }
}

impl From<RoleSelectionMessage> for AppMessage {
    fn from(value: RoleSelectionMessage) -> Self {
        Self::SelectRole(value)
    }
}
