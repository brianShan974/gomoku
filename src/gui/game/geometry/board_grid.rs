use iced::{
    widget::canvas::{Frame, Path, Stroke},
    Point,
};

use crate::game_objects::{Pos, BOARD_SIDELENGTH};

use super::{MARGIN_WIDTH, STROKE_WIDTH, TILE_WIDTH};

pub struct BoardGrid<'a> {
    frame: &'a mut Frame,
}

impl<'a> BoardGrid<'a> {
    pub fn new(frame: &'a mut Frame) -> Self {
        Self { frame }
    }

    pub fn draw(&mut self) {
        let stroke = Stroke {
            width: STROKE_WIDTH as f32,
            ..Stroke::default()
        };

        for i in 0..BOARD_SIDELENGTH {
            let v_start = Self::get_start_of_vertical_line_n(i);
            let v_end = Self::get_end_of_vertical_line_n(i);
            let h_start = Self::get_start_of_horizontal_line_n(i);
            let h_end = Self::get_end_of_horizontal_line_n(i);
            let v_line = Path::line(v_start, v_end);
            let h_line = Path::line(h_start, h_end);

            self.frame.stroke(&v_line, stroke.clone());
            self.frame.stroke(&h_line, stroke.clone());
        }
    }

    fn get_point_on_pos(pos: Pos) -> Point {
        let Point { x, y: _ } = Self::get_start_of_vertical_line_n(pos.0);
        let Point { x: _, y } = Self::get_start_of_horizontal_line_n(pos.1);

        Point { x, y }
    }

    // all 4 methods below require n to start from 0
    // n should be < 15
    fn get_start_of_vertical_line_n(n: usize) -> Point {
        let base_offset = MARGIN_WIDTH;
        Point {
            x: (n * (TILE_WIDTH + STROKE_WIDTH) + base_offset) as f32,
            y: MARGIN_WIDTH as f32,
        }
    }

    fn get_end_of_vertical_line_n(n: usize) -> Point {
        let base_offset = MARGIN_WIDTH;
        Point {
            x: (n * (TILE_WIDTH + STROKE_WIDTH) + base_offset) as f32,
            y: ((BOARD_SIDELENGTH - 1) * (TILE_WIDTH + STROKE_WIDTH) + base_offset) as f32,
        }
    }

    fn get_start_of_horizontal_line_n(n: usize) -> Point {
        let base_offset = MARGIN_WIDTH;
        Point {
            x: MARGIN_WIDTH as f32,
            y: (n * (TILE_WIDTH + STROKE_WIDTH) + base_offset) as f32,
        }
    }

    fn get_end_of_horizontal_line_n(n: usize) -> Point {
        let base_offset = MARGIN_WIDTH;
        Point {
            x: ((BOARD_SIDELENGTH - 1) * (TILE_WIDTH + STROKE_WIDTH) + base_offset) as f32,
            y: (n * (TILE_WIDTH + STROKE_WIDTH) + base_offset) as f32,
        }
    }
}
