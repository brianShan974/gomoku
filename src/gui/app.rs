use core::panic;
use std::sync::Arc;

use iced::{
    executor,
    theme::Theme,
    window::{self, Id},
    Application, Command, Element, Renderer,
};
use tokio::net::TcpStream;

use crate::gui::{
    app_message::AppMessage,
    connecting::message::{ClientConnectingMessage, ConnectingMessage, ServerConnectingMessage},
    network_handler::NetworkHandler,
    role_selection::scene::{Role, RoleSelectionScene},
    scene::{Scene, SceneType, SceneUpdateResult},
};

pub type AppElement<'a> = Element<'a, AppMessage, Theme, Renderer>;
pub type AppCommand = Command<AppMessage>;

pub struct Gomoku {
    current_scene_type: SceneType,
    current_scene: Box<dyn Scene>,
    network_handler: Option<NetworkHandler>,
    // role: Role,
}

impl Application for Gomoku {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let initial_state = Self {
            current_scene_type: SceneType::RoleSelection,
            current_scene: Box::new(RoleSelectionScene),
            network_handler: None,
            // role: Role::Undetermined,
        };

        (initial_state, Command::none())
    }

    fn title(&self) -> String {
        String::from("A Gomoku Game")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::Exit => window::close(Id::MAIN),
            // AppMessage::Connecting(ConnectingMessage::Client(ref msg)) => match msg {
            //     ClientConnectingMessage::Connect(socket_addr) => {
            //         if self.network_handler.is_none() {
            //             Command::perform(TcpStream::connect(socket_addr.to_string()), |s| match s {
            //                 Ok(s) => AppMessage::Connecting(ConnectingMessage::Client(
            //                     ClientConnectingMessage::Connected(Arc::new(s)),
            //                 )),
            //                 Err(_) => AppMessage::Connecting(ConnectingMessage::Client(
            //                     ClientConnectingMessage::ConnectionFailed,
            //                 )),
            //             })
            //         } else {
            //             Command::none()
            //         }
            //     }
            //     ClientConnectingMessage::Connected(s) => {
            //         self.network_handler = Some(NetworkHandler::Client(s.clone()));
            //         Command::none()
            //     }
            //     ClientConnectingMessage::EditAddress(action) => {
            //         match self.current_scene.update(message) {
            //             SceneUpdateResult::CommandOnly(command) => command,
            //             _ => unreachable!(),
            //         }
            //     }
            //     ClientConnectingMessage::ConnectionFailed => unimplemented!(),
            // },
            AppMessage::Connecting(message) => self.handle_connection(message),
            _ => {
                let result = self.current_scene.update(message);
                self.handle_scene_update_result(result)
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        self.current_scene.view()
    }
}

impl Gomoku {
    fn handle_scene_update_result(&mut self, result: SceneUpdateResult) -> Command<AppMessage> {
        match result {
            SceneUpdateResult::CommandOnly(command) => command,
            SceneUpdateResult::SceneSwitch(old_scene_type, new_scene_type, scene, command) => {
                self.current_scene_type = new_scene_type;
                self.current_scene = scene;
                // match (old_scene_type, new_scene_type) {
                //     (SceneType::Connecting(_), SceneType::RoleSelection) => {
                //         self.network_handler = None
                //     }
                //     _ => {}
                // }
                if let (SceneType::Connecting(_), SceneType::RoleSelection) =
                    (old_scene_type, new_scene_type)
                {
                    self.network_handler = None;
                }
                command
            }
        }
    }

    fn handle_connection(&mut self, message: ConnectingMessage) -> Command<AppMessage> {
        match message {
            ConnectingMessage::Client(message) => self.handle_client_connection(message),
            ConnectingMessage::Server(message) => self.handle_server_connection(message),
            ConnectingMessage::Return => {
                let result = self.current_scene.update(message.into());
                self.handle_scene_update_result(result)
            }
        }
    }

    fn handle_client_connection(
        &mut self,
        message: ClientConnectingMessage,
    ) -> Command<AppMessage> {
        match message {
            ClientConnectingMessage::Connect(socket_addr) => {
                if self.network_handler.is_none() {
                    Command::perform(TcpStream::connect(socket_addr), |s| match s {
                        Ok(s) => AppMessage::Connecting(ConnectingMessage::Client(
                            ClientConnectingMessage::Connected(Arc::new(s)),
                        )),
                        Err(e) => AppMessage::Connecting(ConnectingMessage::Client(
                            ClientConnectingMessage::ConnectionFailed(e.to_string()),
                        )),
                    })
                } else {
                    Command::none()
                }
            }
            ClientConnectingMessage::Connected(ref s) => {
                self.network_handler = Some(NetworkHandler::Client(s.clone()));
                let result = self.current_scene.update(message.into());
                self.handle_scene_update_result(result)
            }
            // ClientConnectingMessage::ConnectionFailed => unimplemented!(),
            ClientConnectingMessage::ConnectionFailed(_)
            | ClientConnectingMessage::EditAddress(_) => {
                let result = self.current_scene.update(message.into());
                self.handle_scene_update_result(result)
            }
        }
    }

    fn handle_server_connection(
        &mut self,
        message: ServerConnectingMessage,
    ) -> Command<AppMessage> {
        unimplemented!()
    }
}
