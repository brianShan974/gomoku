use std::sync::Arc;

use iced::widget::text_editor::Action;
use tokio::net::{TcpListener, TcpStream};

use crate::gui::{
    app_message::AppMessage,
    network::server_scene::Port,
    network_handler::{LockedListener, LockedStream},
};

#[derive(Debug, Clone)]
pub enum ConnectingMessage {
    Client(ClientConnectingMessage),
    Server(ServerConnectingMessage),
    Return,
}

#[derive(Debug, Clone)]
pub enum ClientConnectingMessage {
    EditAddress(Action),
    Connect(String),
    Connected(LockedStream),
    ConnectionFailed(String),
}

#[derive(Debug, Clone)]
pub enum ServerConnectingMessage {
    PortBound(LockedListener),
    Connected(LockedStream),
}

impl From<ConnectingMessage> for AppMessage {
    fn from(value: ConnectingMessage) -> Self {
        Self::Connecting(value)
    }
}

impl From<ClientConnectingMessage> for AppMessage {
    fn from(value: ClientConnectingMessage) -> Self {
        Self::Connecting(ConnectingMessage::Client(value))
    }
}

impl From<ServerConnectingMessage> for AppMessage {
    fn from(value: ServerConnectingMessage) -> Self {
        Self::Connecting(ConnectingMessage::Server(value))
    }
}
