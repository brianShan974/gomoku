use iced::widget::canvas::Frame;

use crate::game_objects::{piece::Color, Pos};

pub struct ChessPieceWidget<'a> {
    color: Color,
    pos: Pos,
    frame: &'a mut Frame,
}

impl<'a> ChessPieceWidget<'a> {
    pub fn new(color: Color, pos: Pos, frame: &'a mut Frame) -> Self {
        Self { color, pos, frame }
    }

    pub fn draw(&self) {
        unimplemented!()
    }
}
