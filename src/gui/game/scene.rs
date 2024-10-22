use std::{borrow::BorrowMut, sync::Arc};

use iced::{
    widget::{
        canvas,
        canvas::{Fill, Program},
        container, horizontal_space, row, Text,
    },
    Length,
};
use tokio::{
    net::{
        tcp::{ReadHalf, WriteHalf},
        TcpStream,
    },
    sync::Mutex,
};

use crate::{
    game_objects::{board::Board, piece::Color},
    gui::{
        app::{AppCommand, AppElement},
        app_message::AppMessage,
        network_handler::LockedStream,
        scene::{Scene, SceneUpdateResult},
    },
};

use super::{geometry::gameboard::GameBoard, message::GameMessage};

// pub const LAYOUT: (f32, f32, f32) = (0.375, 1.0, 0.375);

#[derive(Debug)]
pub struct GameScene<'a> {
    game_board: Board,
    color: Color,
    opponent_updated: bool,
    network_reader: ReadHalf<'a>,
    network_sender: WriteHalf<'a>,
}

impl<'a> Scene for GameScene<'a> {
    fn view(&self) -> AppElement<'_> {
        canvas(GameBoard::default()).height(Length::Fill).into()
    }

    fn update(&mut self, message: AppMessage) -> SceneUpdateResult {
        if let AppMessage::Game(game_message) = message {
            match game_message {
                GameMessage::SelfPlayed(next_move) => {
                    if self.opponent_updated {
                        self.game_board.apply_move(next_move.clone());
                        self.opponent_updated = false;

                        SceneUpdateResult::CommandOnly(AppCommand::perform(
                            async move {},
                            move |()| GameMessage::OpponentPlayed(next_move).into(),
                        )) // send an OpponentPlayed
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

impl<'a> GameScene<'a> {
    pub fn new(stream: Arc<Mutex<TcpStream>>, color: Color) -> Self {
        let (network_reader, network_sender) = stream.lock().await;
        Self {
            game_board: Board::default(),
            color,
            opponent_updated: true,
            network_reader,
            network_sender,
        }
    }
}
