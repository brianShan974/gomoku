use iced::widget::{button, column, row, Text};

use crate::gui::app::{ElementType, GUIMessage, Role};

#[derive(Debug, Default)]
pub struct RoleSelectionScene;

impl RoleSelectionScene {
    pub fn view(&self) -> ElementType<'_> {
        let user_hint = Text::new("Please select whether you want to be the client or the server:");

        let server_button = button("SERVER").on_press(GUIMessage::SelectRole(Role::Server));
        let client_button = button("CLIENT").on_press(GUIMessage::SelectRole(Role::Client));
        let buttons = row!(server_button, client_button);

        let exit_button = button("EXIT").on_press(GUIMessage::Exit);

        column!(user_hint, buttons, exit_button).into()
    }
}
