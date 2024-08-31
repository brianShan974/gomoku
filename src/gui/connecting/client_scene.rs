use iced::{
    alignment::Horizontal,
    widget::{
        button, column, row, text_editor,
        text_editor::{Action, Content, Edit},
        Text,
    },
};

use std::net::SocketAddr;

use crate::gui::{
    app::{AppCommand, AppElement},
    app_message::AppMessage,
    connecting::message::{ClientConnectingMessage, ConnectingMessage},
    connection_state::ConnectionState,
    game::scene::GameScene,
    role_selection::scene::{Role, RoleSelectionScene},
    scene::{Scene, SceneType, SceneUpdateResult},
};

#[derive(Debug, Default)]
pub enum ClientConnectingSceneState {
    #[default]
    Normal,
    Error(String),
}

#[derive(Debug, Default)]
pub struct ClientConnectingScene {
    input_content: Content,
    state: ClientConnectingSceneState,
}

impl Scene for ClientConnectingScene {
    fn view(&self) -> AppElement<'_> {
        let user_hint = match &self.state {
            ClientConnectingSceneState::Normal => column!(
                Text::new("Please enter a valid IP address and a port."),
                Text::new(r#"If it is an IPv4 address, please format it as "<ip>:<port>"."#),
                Text::new(r#"If it is an IPv6 address, please format it as "[<ip>]:<port>"."#),
            ),
            ClientConnectingSceneState::Error(msg) => column!(
                Text::new("Connection failed!"),
                Text::new("Error message:"),
                Text::new(msg),
                Text::new("There are 3 possibilities:"),
                Text::new("1. Your IP address and port may be in an incorrect format."),
                Text::new(r#"If it is an IPv4 address, please format it as "<ip>:<port>"."#),
                Text::new(r#"If it is an IPv6 address, please format it as "[<ip>]:<port>"."#),
                Text::new("2. The IP address is not pointing to a working server."),
                Text::new("3. The server is not listening on you port."),
            ),
        };

        let editor = text_editor(&self.input_content).on_action(|action| {
            match action {
                Action::Edit(Edit::Enter) => {
                    ClientConnectingMessage::Connect(self.input_content.text().trim().to_string())
                }
                _ => ClientConnectingMessage::EditAddress(action),
            }
            .into()
        });

        let connect_button = button("CONNECT").on_press(
            ClientConnectingMessage::Connect(self.input_content.text().trim().to_string()).into(),
        );
        let exit_button = button("EXIT").on_press(AppMessage::Exit);
        let return_button = button("GO BACK").on_press(ConnectingMessage::Return.into());

        let buttons = row!(connect_button, exit_button, return_button);

        column!(user_hint, editor, buttons)
            .align_items(Horizontal::Center.into())
            .into()
    }

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult {
        match message {
            AppMessage::Connecting(ConnectingMessage::Client(message)) => match message {
                ClientConnectingMessage::EditAddress(action) => {
                    self.input_content.perform(action);
                }
                ClientConnectingMessage::ConnectionFailed(msg) => {
                    self.state = ClientConnectingSceneState::Error(msg)
                }
                ClientConnectingMessage::Connected(_) => {
                    return SceneUpdateResult::SceneSwitch(
                        SceneType::Connecting(Role::Client),
                        SceneType::Game,
                        Box::new(GameScene::default()),
                        AppCommand::none(),
                    )
                }
                _ => unreachable!(),
            },
            AppMessage::Connecting(ConnectingMessage::Return) => {
                return SceneUpdateResult::SceneSwitch(
                    SceneType::Connecting(Role::Client),
                    SceneType::RoleSelection,
                    Box::new(RoleSelectionScene),
                    AppCommand::none(),
                )
            }
            _ => unimplemented!(),
        }
        SceneUpdateResult::CommandOnly(AppCommand::none())
    }
}

impl ClientConnectingScene {}
