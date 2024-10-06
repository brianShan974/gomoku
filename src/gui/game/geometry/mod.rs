use iced::Size;

use crate::game_objects::BOARD_SIDELENGTH;

pub mod board_grid;
pub mod chesspiece_widget;

pub mod gameboard;

pub(super) const TILE_WIDTH: usize = 20;
pub(super) const STROKE_WIDTH: usize = 2;
pub(super) const MARGIN_WIDTH: usize = 20;

pub(super) const BOARD_SIZE: Size = Size::new(
    (STROKE_WIDTH * (BOARD_SIDELENGTH + 1) + TILE_WIDTH * BOARD_SIDELENGTH + MARGIN_WIDTH) as f32,
    (STROKE_WIDTH * (BOARD_SIDELENGTH + 1) + TILE_WIDTH * BOARD_SIDELENGTH + MARGIN_WIDTH) as f32,
);
