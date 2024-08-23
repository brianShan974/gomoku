use iced::{
    widget::{
        button, column, row, text_editor,
        text_editor::{Action, Content, Edit},
        Text,
    },
    Command,
};

use std::net::SocketAddr;

use crate::gui::{
    app::{AppElement, AppMessage},
    connecting::message::ConnectingMessage,
};

#[derive(Debug, Default)]
enum ClientStartingSceneState {
    #[default]
    Default,
    Error,
}

#[derive(Debug, Default)]
pub struct ClientConnectingScene {
    input_content: Content,
    addr: Option<SocketAddr>,
    state: ClientStartingSceneState,
}

impl ClientConnectingScene {
    pub fn view(&self) -> AppElement<'_> {
        let user_hint = match self.state {
            ClientStartingSceneState::Default => column!(
                Text::new("Please enter a valid IP address and a port."),
                Text::new(r#"If it is an IPv4 address, please format it as "<ip>:<port>"."#),
                Text::new(r#"If it is an IPv6 address, please format it as "[<ip>]:<port>"."#),
            ),
            ClientStartingSceneState::Error => column!(
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

        column!(user_hint, editor, buttons).into()
    }

    pub fn update(&mut self, message: ConnectingMessage) -> Command<ConnectingMessage> {
        if let ConnectingMessage::Edit(action) = message {
            if let Action::Edit(Edit::Enter) = action {
                match SocketAddr::parse_ascii(self.input_content.text().as_bytes()) {
                    Ok(addr) => {
                        self.addr = Some(addr);
                    }
                    Err(_) => {
                        self.state = ClientStartingSceneState::Error;
                    }
                }
            }
            self.input_content.perform(action.clone());
        }
        Command::none()
    }

    pub fn get_addr(&self) -> Option<SocketAddr> {
        self.addr
    }
}
