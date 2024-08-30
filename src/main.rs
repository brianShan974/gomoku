use iced::{Application, Settings};

use crate::gui::app::Gomoku;
mod game_objects;
mod gui;

fn main() -> Result<(), iced::Error> {
    Gomoku::run(Settings::default())
}
