use iced::{executor, theme, Application, Command};

use crate::gui::{
    game_scene::{GameScene, GomokuMessage},
    menu_scene::{MenuMessage, MenuScene},
    role_selection_scene::{RoleSelectionMessage, RoleSelectionScene},
    starting_scenes::starting_scene::{StartingMessage, StartingScene},
};

pub type ElementType<'a> = iced::Element<'a, GUIMessage, theme::Theme, iced::Renderer>;
pub type AppCommand = Command<GUIMessage>;

#[derive(Debug, Default)]
pub struct Gomoku {
    screen: Screen,
    connection_state: ConnectionState,
}

impl Application for Gomoku {
    type Executor = executor::Default;
    type Message = GUIMessage;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("A Gomoku Game")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            GUIMessage::Start(starting_message) => {
                if let Screen::Start(ref mut starting_scene) = &mut self.screen {
                    starting_scene.update(starting_message)
                } else {
                    Command::none()
                }
            }
            GUIMessage::Game(gomoku_message) => {
                if let Screen::Game(ref mut gomoku_scene) = &mut self.screen {
                    gomoku_scene.update(gomoku_message)
                } else {
                    Command::none()
                }
            }
            GUIMessage::Menu(menu_message) => {
                if let Screen::Paused(ref mut menu_scene) = &mut self.screen {
                    menu_scene.update(menu_message)
                } else {
                    Command::none()
                }
            }
            GUIMessage::SelectRole(role_selection_message) => {
                if let Screen::RoleSelection(ref mut role_selection_scene) = &mut self.screen {
                    role_selection_scene.update(role_selection_message)
                } else {
                    Command::none()
                }
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        match &self.screen {
            Screen::Start(StartingScene::Client(scene)) => scene.view(),
            Screen::Start(StartingScene::Server(scene)) => scene.view(),
            Screen::Paused(scene) => scene.view(),
            Screen::Game(scene) => scene.view(),
            Screen::RoleSelection(scene) => scene.view(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GUIMessage {
    SelectRole(RoleSelectionMessage),
    Start(StartingMessage),
    Game(GomokuMessage),
    Menu(MenuMessage),
    Nothing,
    Exit,
}

#[derive(Debug)]
pub enum Screen {
    Start(StartingScene),
    Game(GameScene),
    Paused(MenuScene),
    RoleSelection(RoleSelectionScene),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Start(StartingScene::default())
    }
}

#[derive(Debug, Default)]
pub enum ConnectionState {
    Connected,
    #[default]
    NotConnected,
}
