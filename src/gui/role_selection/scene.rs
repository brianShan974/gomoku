use iced::{
    alignment::Horizontal,
    widget::{button, column, row, Text},
};

use crate::gui::{
    app::{AppCommand, AppElement},
    app_message::AppMessage,
    scene::{Scene, UpdateResult},
};

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

impl Scene for RoleSelectionScene {
    fn view(&self) -> AppElement<'_> {
        let user_hint = Text::new("Please select whether you want to be the client or the server:");

        let server_button = button("SERVER").on_press(RoleSelectionMessage::ChooseServer.into());
        let client_button = button("CLIENT").on_press(RoleSelectionMessage::ChooseClient.into());
        let buttons = row!(server_button, client_button);

        let exit_button = button("EXIT").on_press(AppMessage::Exit);

        column!(user_hint, buttons, exit_button)
            .align_items(Horizontal::Center.into())
            .into()
    }

    fn update(&mut self, message: AppMessage) -> UpdateResult {
        unimplemented!()
    }
}

impl From<RoleSelectionMessage> for AppMessage {
    fn from(value: RoleSelectionMessage) -> Self {
        Self::SelectRole(value)
    }
}
