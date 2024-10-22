use std::sync::Arc;

use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

pub type LockedStream = Arc<Mutex<TcpStream>>;
pub type LockedListener = Arc<TcpListener>;

#[derive(Clone)]
pub enum NetworkHandler {
    Client(LockedStream),
    Server(LockedListener),
    ServerConnected(LockedStream),
}
