use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};

#[derive(Clone)]
pub enum NetworkHandler {
    Client(Arc<TcpStream>),
    Server(Arc<TcpListener>),
    ServerConnected(Arc<TcpStream>),
}
