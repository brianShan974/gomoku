use std::marker::PhantomData;

use iced::{
    mouse::Cursor,
    widget::canvas::{Frame, Program},
    Rectangle,
};

use crate::gui::{
    app::{AppRenderer, AppTheme},
    app_message::AppMessage,
    game::geometry::board_grid::BoardGrid,
};

use super::BOARD_SIZE;

#[derive(Default)]
pub struct GameBoard {}

impl Program<AppMessage, AppTheme, AppRenderer> for GameBoard {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        renderer: &AppRenderer,
        theme: &AppTheme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<<AppRenderer as iced::widget::canvas::Renderer>::Geometry> {
        let mut frame = Frame::new(renderer, BOARD_SIZE);

        let mut grid = BoardGrid::new(&mut frame);

        grid.draw();

        vec![frame.into_geometry()]
    }
}
