use iced::{
    executor,
    theme::Theme,
    widget::text_editor::Content,
    window::{self, Id},
    Application, Command, Element, Renderer,
};

use crate::gui::{
    app_message::AppMessage,
    connecting::{
        client_scene::ClientConnectingScene, message::ConnectingMessage,
        server_scene::ServerConnectingScene,
    },
    game::{message::GameMessage, scene::GameScene},
    menu::scene::{MenuMessage, MenuScene},
    role_selection::scene::{Role, RoleSelectionMessage, RoleSelectionScene},
    scene::{Scene, SceneType, UpdateResult},
};

pub type AppElement<'a> = Element<'a, AppMessage, Theme, Renderer>;
pub type AppCommand = Command<AppMessage>;

pub struct Gomoku {
    current_scene_type: SceneType,
    current_scene: Box<dyn Scene>,
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
        };

        (initial_state, Command::none())
    }

    fn title(&self) -> String {
        String::from("A Gomoku Game")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self.current_scene.update(message) {
            UpdateResult::Command(command) => command,
            UpdateResult::SceneSwitch(scene_type, scene, command) => {
                self.current_scene_type = scene_type;
                self.current_scene = scene;
                command
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        match &self.current_scene {
            _ => todo!(),
        }
    }
}

impl Gomoku {}
