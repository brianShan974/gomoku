use tokio::net::{TcpListener, TcpStream};

pub enum NetworkHandler {
    Client(TcpStream),
    Server(TcpListener),
}
