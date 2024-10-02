use std::sync::Arc;

use iced::{
    alignment::Horizontal,
    widget::{button, column, row, Text},
};
use tokio::net::TcpListener;

use crate::gui::{
    app::{AppCommand, AppElement},
    app_message::AppMessage,
    network::{
        client_scene::ClientConnectingScene,
        message::{ConnectingMessage, ServerConnectingMessage},
        server_scene::{get_available_port, ServerConnectingScene, MAX_PORT, MIN_PORT},
    },
    role_selection::message::RoleSelectionMessage,
    scene::{Scene, SceneType, SceneUpdateResult},
};

#[derive(Debug, Default)]
pub struct RoleSelectionScene;

#[derive(Debug, Clone, Copy, Default)]
pub enum Role {
    #[default]
    Undetermined,
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
            let old_scene_type = SceneType::RoleSelection;
            match message {
                RoleSelectionMessage::ChooseClient => SceneUpdateResult::SceneSwitch(
                    old_scene_type,
                    SceneType::Connecting(Role::Client),
                    Box::new(ClientConnectingScene::default()),
                    AppCommand::none(),
                ),
                RoleSelectionMessage::ChooseServer => SceneUpdateResult::SceneSwitch(
                    old_scene_type,
                    SceneType::Connecting(Role::Server),
                    Box::new(ServerConnectingScene::default()),
                    AppCommand::perform(
                        TcpListener::bind((
                            "127.0.0.1",
                            get_available_port(MIN_PORT, MAX_PORT).unwrap(),
                        )),
                        |result| match result {
                            Ok(listener) => ConnectingMessage::Server(
                                ServerConnectingMessage::PortBound(Arc::new(listener)),
                            )
                            .into(),
                            Err(_) => RoleSelectionMessage::ChooseServer.into(),
                        },
                    ),
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
