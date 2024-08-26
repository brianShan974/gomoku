use core::panic;

use iced::{
    alignment::Horizontal,
    widget::{button, column, row, Text},
    Command,
};

use crate::gui::{
    app::{AppCommand, AppElement},
    app_message::AppMessage,
    connecting::{client_scene::ClientConnectingScene, server_scene::ServerConnectingScene},
    scene::{Scene, SceneType, SceneUpdateResult},
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

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult {
        if let AppMessage::SelectRole(message) = message {
            let command = Command::none();
            match message {
                RoleSelectionMessage::ChooseClient => SceneUpdateResult::SceneSwitch(
                    SceneType::Connecting(Role::Client),
                    Box::new(ClientConnectingScene::default()),
                    command,
                ),
                RoleSelectionMessage::ChooseServer => SceneUpdateResult::SceneSwitch(
                    SceneType::Connecting(Role::Server),
                    Box::new(ServerConnectingScene::default()),
                    command,
                ),
            }
        } else {
            panic!("Wrong type of message received!")
        }
    }
}

impl From<RoleSelectionMessage> for AppMessage {
    fn from(value: RoleSelectionMessage) -> Self {
        Self::SelectRole(value)
    }
}
