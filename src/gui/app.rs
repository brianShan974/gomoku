use std::{io::Result, net::SocketAddr, sync::Arc};

use iced::{
    executor,
    theme::Theme,
    window::{self, Id},
    Application, Command, Element, Renderer,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use crate::gui::{
    app_message::AppMessage,
    network::{
        message::{ClientConnectingMessage, ConnectingMessage, ServerConnectingMessage},
        server_scene::ServerConnectingScene,
    },
    network_handler::NetworkHandler,
    role_selection::scene::{Role, RoleSelectionScene},
    scene::{Scene, SceneType, SceneUpdateResult},
};

pub type AppElement<'a> = Element<'a, AppMessage, Theme, Renderer>;
pub type AppCommand = Command<AppMessage>;
pub type AppTheme = Theme;
pub type AppRenderer = Renderer;

pub struct Gomoku {
    current_scene_type: SceneType,
    current_scene: Box<dyn Scene>,
    network_handler: Option<NetworkHandler>,
    // role: Role,
}

impl Application for Gomoku {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Theme = AppTheme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, AppCommand) {
        let initial_state = Self {
            current_scene_type: SceneType::RoleSelection,
            current_scene: Box::new(RoleSelectionScene),
            network_handler: None,
            // role: Role::Undetermined,
        };

        (initial_state, AppCommand::none())
    }

    fn title(&self) -> String {
        String::from("A Gomoku Game")
    }

    fn update(&mut self, message: Self::Message) -> AppCommand {
        match message {
            AppMessage::Exit => window::close(Id::MAIN),
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
    fn handle_scene_update_result(&mut self, result: SceneUpdateResult) -> AppCommand {
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
                if let (_, SceneType::RoleSelection) = (old_scene_type, new_scene_type) {
                    self.network_handler = None;
                }
                command
            }
        }
    }

    fn handle_connection(&mut self, message: ConnectingMessage) -> AppCommand {
        match message {
            ConnectingMessage::Client(message) => self.handle_client_connection(message),
            ConnectingMessage::Server(message) => self.handle_server_connection(message),
            ConnectingMessage::Return => {
                let result = self.current_scene.update(message.into());
                self.handle_scene_update_result(result)
            }
        }
    }

    fn handle_client_connection(&mut self, message: ClientConnectingMessage) -> AppCommand {
        match message {
            ClientConnectingMessage::Connect(socket_addr) => {
                if self.network_handler.is_none() {
                    AppCommand::perform(TcpStream::connect(socket_addr), |s| match s {
                        Ok(s) => AppMessage::Connecting(ConnectingMessage::Client(
                            ClientConnectingMessage::Connected(Arc::new(Mutex::new(s))),
                        )),
                        Err(e) => AppMessage::Connecting(ConnectingMessage::Client(
                            ClientConnectingMessage::ConnectionFailed(e.to_string()),
                        )),
                    })
                } else {
                    AppCommand::none()
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

    fn handle_server_connection(&mut self, message: ServerConnectingMessage) -> AppCommand {
        match message {
            ServerConnectingMessage::PortBound(listener) => {
                self.network_handler = Some(NetworkHandler::Server(listener.clone()));
                self.current_scene_type = SceneType::Connecting(Role::Server);
                self.current_scene = Box::new(ServerConnectingScene::new(
                    listener.local_addr().unwrap().port(),
                ));
                let task = async move { listener.accept().await };
                AppCommand::perform(task, |result| match result {
                    Ok((stream, _)) => {
                        ServerConnectingMessage::Connected(Arc::new(Mutex::new(stream))).into()
                    }
                    Err(_) => unimplemented!(),
                })
            }
            ServerConnectingMessage::Connected(ref tcp_stream) => {
                self.network_handler = Some(NetworkHandler::ServerConnected(tcp_stream.clone()));
                let result = self.current_scene.update(message.into());
                self.handle_scene_update_result(result)
            }
        }
    }
}
