use std::sync::Arc;

use iced::widget::text_editor::Action;
use tokio::net::{TcpListener, TcpStream};

use crate::gui::{app_message::AppMessage, network::server_scene::Port};

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
    Connected(Arc<TcpStream>),
    ConnectionFailed(String),
}

#[derive(Debug, Clone)]
pub enum ServerConnectingMessage {
    PortBound(Arc<TcpListener>),
    Connected(Arc<TcpStream>),
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
