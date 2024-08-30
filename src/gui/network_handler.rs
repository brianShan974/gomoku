use std::sync::Arc;

use tokio::net::{TcpListener, TcpStream};

pub enum NetworkHandler {
    Client(Arc<TcpStream>),
    Server(Arc<TcpListener>),
}
