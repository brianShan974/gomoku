use std::{net::TcpListener, u16};

use crate::gui::{
    app::AppElement,
    app_message::AppMessage,
    scene::{Scene, SceneUpdateResult},
};

pub type Port = u16;

#[derive(Debug)]
pub struct ServerConnectingScene {
    port: Port,
}

impl Default for ServerConnectingScene {
    fn default() -> Self {
        Self {
            port: get_available_port(9000, u16::MAX).unwrap(),
        }
    }
}

impl Scene for ServerConnectingScene {
    fn view(&self) -> AppElement<'_> {
        unimplemented!()
    }

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult {
        unimplemented!()
    }
}

fn get_available_port(lower: Port, upper: Port) -> Option<Port> {
    (lower..=upper).find(port_is_available)
}

fn port_is_available(port: &Port) -> bool {
    TcpListener::bind(("127.0.0.1", *port)).is_ok()
}
