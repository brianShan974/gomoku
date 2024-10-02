use std::{net::TcpListener, u16};

use iced::widget::{button, column, Text};

use crate::gui::{
    app::{AppCommand, AppElement},
    app_message::AppMessage,
    game::scene::GameScene,
    network::message::{ConnectingMessage, ServerConnectingMessage},
    role_selection::scene::{Role, RoleSelectionScene},
    scene::{Scene, SceneType, SceneUpdateResult},
};

pub type Port = u16;
pub const MIN_PORT: u16 = 9000u16;
pub const MAX_PORT: u16 = u16::MAX;

#[derive(Debug, Default)]
pub struct ServerConnectingScene {
    port: Port,
}

impl ServerConnectingScene {
    pub fn new(port: Port) -> Self {
        Self { port }
    }
}

impl Scene for ServerConnectingScene {
    fn view(&self) -> AppElement<'_> {
        let return_button = button("GO BACK").on_press(ConnectingMessage::Return.into());

        column!(
            Text::new("Great! Your port is generated!"),
            Text::new(format!("Your server is listening on port {}", self.port)),
            return_button,
        )
        .into()
    }

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult {
        match message {
            AppMessage::Connecting(ConnectingMessage::Server(
                ServerConnectingMessage::Connected(_),
            )) => SceneUpdateResult::SceneSwitch(
                SceneType::Connecting(Role::Server),
                SceneType::Game,
                Box::new(GameScene::default()),
                AppCommand::none(),
            ),
            AppMessage::Connecting(ConnectingMessage::Return) => SceneUpdateResult::SceneSwitch(
                SceneType::Connecting(Role::Server),
                SceneType::RoleSelection,
                Box::new(RoleSelectionScene),
                AppCommand::none(),
            ),
            _ => SceneUpdateResult::default(),
        }
    }
}

pub(in crate::gui) fn get_available_port(lower: Port, upper: Port) -> Option<Port> {
    (lower..=upper).find(port_is_available)
}

fn port_is_available(port: &Port) -> bool {
    TcpListener::bind(("127.0.0.1", *port)).is_ok()
}
