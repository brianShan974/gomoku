use iced::{Application, Settings};

mod game_objects;
mod gui;

use crate::gui::app::Gomoku;

fn main() -> Result<(), iced::Error> {
    Gomoku::run(Settings::default())
}
