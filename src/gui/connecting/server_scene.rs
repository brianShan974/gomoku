use std::net::TcpListener;

use iced::widget::{column, Text};

use crate::gui::{
    app::{AppCommand, AppElement},
    app_message::AppMessage,
    connecting::message::{ConnectingMessage, ServerConnectingMessage},
    game::scene::GameScene,
    role_selection::scene::Role,
    scene::{Scene, SceneType, SceneUpdateResult},
};

pub type Port = u16;

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
        column!(
            Text::new("Great! Your port is generated!"),
            Text::new(format!("Your server is listening on port {}", self.port)),
        )
        .into()
    }

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult {
        if let AppMessage::Connecting(ConnectingMessage::Server(
            ServerConnectingMessage::Connected,
        )) = message
        {
            SceneUpdateResult::SceneSwitch(
                SceneType::Connecting(Role::Server),
                SceneType::Game,
                Box::new(GameScene::default()),
                AppCommand::none(),
            )
        } else {
            SceneUpdateResult::default()
        }
    }
}

pub(in crate::gui) fn get_available_port(lower: Port, upper: Port) -> Option<Port> {
    (lower..=upper).find(port_is_available)
}

fn port_is_available(port: &Port) -> bool {
    TcpListener::bind(("127.0.0.1", *port)).is_ok()
}
