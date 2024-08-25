use iced::{
    executor,
    theme::Theme,
    widget::text_editor::Content,
    window::{self, Id},
    Application, Command, Element, Renderer,
};

use std::net::SocketAddr;

use crate::gui::{
    app_message::AppMessage,
    connecting::{
        client_scene::ClientConnectingScene, message::ConnectingMessage,
        server_scene::ServerConnectingScene,
    },
    game::{message::GameMessage, scene::GameScene},
    menu::scene::{MenuMessage, MenuScene},
    role_selection::scene::{Role, RoleSelectionMessage, RoleSelectionScene},
    scene::{CurrentScene, Scene},
};

pub type AppElement<'a> = Element<'a, AppMessage, Theme, Renderer>;
pub type AppCommand = Command<AppMessage>;

#[derive(Debug, Default)]
pub struct Gomoku {
    current_scene: CurrentScene,
    role: Option<Role>,
}

impl Application for Gomoku {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("A Gomoku Game")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::SelectRole(role_selection_message) => {
                self.process_role_selection_message(role_selection_message)
            }
            AppMessage::Connecting(connecting_message) => {
                self.process_connecting_message(connecting_message)
            }
            AppMessage::Game(game_message) => self.process_game_message(game_message),
            AppMessage::Menu(menu_message) => self.process_menu_message(menu_message),
            AppMessage::Exit => window::close::<AppMessage>(Id::MAIN),
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        match &self.current_scene {
            _ => todo!(),
        }
    }
}

impl Gomoku {
    fn construct_scene(&self) -> Box<dyn Scene> {
        match self.current_scene {
            CurrentScene::RoleSelection => self.construct_role_selection_scene(),
            CurrentScene::Connecting(Role::Client) => self.construct_client_connecting_scene(),
            CurrentScene::Connecting(Role::Server) => self.construct_server_connecting_scene(),
            CurrentScene::Game => self.construct_game_scene(),
            CurrentScene::Menu => self.construct_menu_scene(),
        }
    }

    fn construct_role_selection_scene(&self) -> Box<RoleSelectionScene> {
        Box::new(RoleSelectionScene)
    }

    fn construct_client_connecting_scene(&self) -> Box<ClientConnectingScene> {
        unimplemented!()
    }

    fn construct_server_connecting_scene(&self) -> Box<ServerConnectingScene> {
        unimplemented!()
    }

    fn construct_game_scene(&self) -> Box<GameScene> {
        unimplemented!()
    }

    fn construct_menu_scene(&self) -> Box<MenuScene> {
        unimplemented!()
    }

    fn process_role_selection_message(
        &mut self,
        message: RoleSelectionMessage,
    ) -> Command<AppMessage> {
        self.role = match message {
            RoleSelectionMessage::ChooseClient => Some(Role::Client),
            RoleSelectionMessage::ChooseServer => Some(Role::Server),
        };
        Command::none()
    }

    fn process_connecting_message(&mut self, message: ConnectingMessage) -> Command<AppMessage> {
        unimplemented!()
    }

    fn process_game_message(&mut self, message: GameMessage) -> Command<AppMessage> {
        unimplemented!()
    }

    fn process_menu_message(&mut self, message: MenuMessage) -> Command<AppMessage> {
        unimplemented!()
    }
}
