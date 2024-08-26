use iced::{
    alignment::Horizontal,
    widget::{
        button, column, row, text_editor,
        text_editor::{Action, Content, Edit},
        Text,
    },
    Command,
};

use std::net::SocketAddr;

use crate::gui::{
    app::{AppCommand, AppElement},
    app_message::AppMessage,
    connecting::message::ConnectingMessage,
    scene::{Scene, SceneUpdateResult},
};

#[derive(Debug, Default)]
pub enum ClientConnectingSceneState {
    #[default]
    Default,
    Error,
}

#[derive(Debug, Default)]
pub struct ClientConnectingScene {
    input_content: Content,
    addr: Option<SocketAddr>,
    state: ClientConnectingSceneState,
}

impl ClientConnectingScene {
    pub fn get_addr(&self) -> Option<SocketAddr> {
        self.addr
    }
}

impl Scene for ClientConnectingScene {
    fn view(&self) -> AppElement<'_> {
        let user_hint = match self.state {
            ClientConnectingSceneState::Default => column!(
                Text::new("Please enter a valid IP address and a port."),
                Text::new(r#"If it is an IPv4 address, please format it as "<ip>:<port>"."#),
                Text::new(r#"If it is an IPv6 address, please format it as "[<ip>]:<port>"."#),
            ),
            ClientConnectingSceneState::Error => column!(
                Text::new("Your IP address and port are in an incorrect format."),
                Text::new("Please enter a correct one:"),
            ),
        };

        let editor = text_editor(&self.input_content)
            .on_action(|action| ConnectingMessage::Edit(action).into());

        let connect_button = button("CONNECT").on_press(ConnectingMessage::Connect.into());
        let exit_button = button("EXIT").on_press(AppMessage::Exit);
        let return_button = button("GO BACK").on_press(ConnectingMessage::Return.into());

        let buttons = row!(connect_button, exit_button, return_button);

        column!(user_hint, editor, buttons)
            .align_items(Horizontal::Center.into())
            .into()
    }

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult {
        if let AppMessage::Connecting(ConnectingMessage::Edit(action)) = message {
            if let Action::Edit(Edit::Enter) = action {
                match SocketAddr::parse_ascii(self.input_content.text().as_bytes()) {
                    Ok(addr) => {
                        self.addr = Some(addr);
                    }
                    Err(_) => {
                        self.state = ClientConnectingSceneState::Error;
                    }
                }
            }
            self.input_content.perform(action.clone());
        }
        SceneUpdateResult::CommandOnly(Command::none())
    }
}
