use crate::gui::{
    app::AppElement,
    app_message::AppMessage,
    scene::{Scene, SceneUpdateResult},
};

pub type Port = u16;

#[derive(Debug, Default)]
pub struct ServerConnectingScene {
    port: Port,
}

impl Scene for ServerConnectingScene {
    fn view(&self) -> AppElement<'_> {
        unimplemented!()
    }

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult {
        unimplemented!()
    }
}
