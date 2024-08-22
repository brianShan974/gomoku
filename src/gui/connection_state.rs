#[derive(Debug, Default)]
pub enum ConnectionState {
    Connected,
    #[default]
    NotConnected,
}
