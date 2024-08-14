use iced::{executor, theme, Application, Command};

use crate::gui::{
    game_scene::{GameScene, GomokuMessage},
    menu_scene::{MenuScene, SettingMessage},
    starting_scenes::{client_starting_scene::StartingMessage, starting_scene::StartingScene},
};

pub type ElementType<'a> = iced::Element<'a, GUIMessage, theme::Theme, iced::Renderer>;

pub const LAYOUT: (f32, f32, f32) = (0.375, 1.0, 0.375);

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

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        match &self.screen {
            Screen::Start(StartingScene::Client(scene)) => scene.view(),
            Screen::Start(StartingScene::Server(scene)) => scene.view(),
            Screen::Paused(scene) => scene.view(),
            Screen::Game(scene) => scene.view(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GUIMessage {
    SelectRole(Role),
    Start(StartingMessage),
    Game(GomokuMessage),
    Setting(SettingMessage),
    Nothing,
    Exit,
}

#[derive(Debug)]
pub enum Screen {
    Start(StartingScene),
    Game(GameScene),
    Paused(MenuScene),
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

#[derive(Debug, Clone)]
pub enum Role {
    Client,
    Server,
}
