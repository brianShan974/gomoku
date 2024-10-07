use std::sync::Arc;

use iced::{
    widget::{
        canvas,
        canvas::{Fill, Program},
        container, horizontal_space, row, Text,
    },
    Length,
};
use tokio::net::TcpStream;

use crate::{
    game_objects::{board::Board, piece::Color},
    gui::{
        app::{AppCommand, AppElement},
        app_message::AppMessage,
        scene::{Scene, SceneUpdateResult},
    },
};

use super::{geometry::gameboard::GameBoard, message::GameMessage};

// pub const LAYOUT: (f32, f32, f32) = (0.375, 1.0, 0.375);

#[derive(Debug)]
pub struct GameScene {
    game_board: Board,
    stream: Arc<TcpStream>,
    color: Color,
    opponent_updated: bool,
}

impl Scene for GameScene {
    fn view(&self) -> AppElement<'_> {
        canvas(GameBoard::default()).height(Length::Fill).into()
    }

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult {
        if let AppMessage::Game(game_message) = message {
            match game_message {
                GameMessage::SelfPlayed(next_move) => {
                    if self.opponent_updated {
                        self.game_board.apply_move(next_move);
                        self.opponent_updated = false;
                        SceneUpdateResult::CommandOnly(unimplemented!()) // send an OpponentPlayed
                                                                         // message to the opponent
                    } else {
                        SceneUpdateResult::CommandOnly(AppCommand::none())
                    }
                }
                GameMessage::OpponentPlayed(next_move) => {
                    self.game_board.apply_move(next_move);
                    SceneUpdateResult::CommandOnly(unimplemented!()) // send a message when
                                                                     // updating is done
                }
                GameMessage::OpponentUpdated => {
                    self.opponent_updated = true;
                    SceneUpdateResult::CommandOnly(AppCommand::none())
                }
                _ => unimplemented!(),
            }
        } else {
            SceneUpdateResult::CommandOnly(AppCommand::none())
        }
    }
}

impl GameScene {
    pub fn new(stream: Arc<TcpStream>, color: Color) -> Self {
        Self {
            game_board: Board::default(),
            stream,
            color,
            opponent_updated: true,
        }
    }
}
