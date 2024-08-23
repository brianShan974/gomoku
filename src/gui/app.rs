use iced::{
    executor,
    theme::Theme,
    window::{self, Id},
    Application, Command, Element, Renderer,
};

use crate::gui::{
    connecting::{message::ConnectingMessage, scene::ConnectingScene},
    connection_state::ConnectionState,
    game::message::GameMessage,
    menu::scene::MenuMessage,
    role_selection::scene::{Role, RoleSelectionMessage},
    scene::{CurrentScene, Scene},
};

pub type AppElement<'a> = Element<'a, AppMessage, Theme, Renderer>;
pub type AppCommand = Command<AppMessage>;

#[derive(Debug, Clone)]
pub enum AppMessage {
    SelectRole(RoleSelectionMessage),
    Connecting(ConnectingMessage),
    Game(GameMessage),
    Menu(MenuMessage),
    // Nothing,
    Exit,
}

#[derive(Debug, Default)]
pub struct Gomoku {
    current_scene: CurrentScene,
    role: Role,
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
        unimplemented!()
    }

    fn process_role_selection_message(
        &mut self,
        message: RoleSelectionMessage,
    ) -> Command<AppMessage> {
        self.role = match message {
            RoleSelectionMessage::ChooseClient => Role::Client,
            RoleSelectionMessage::ChooseServer => Role::Server,
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
