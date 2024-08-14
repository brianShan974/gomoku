use crate::gui::app::ElementType;

#[derive(Debug, Clone)]
pub enum SettingMessage {}

#[derive(Debug)]
pub struct MenuScene {}

impl MenuScene {
    pub fn view(&self) -> ElementType<'_> {
        unimplemented!()
    }
}
