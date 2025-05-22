#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bitboard;
mod terminal;
mod minimax;

use minimax::minimax;
use bitboard::{BitBoard, Piece};
use eframe::egui;

const CELL_SIZE: f32 = 50.0;
const CELL_SPACING: f32 = 5.0;

struct Connect4App {
    game_state: BitBoard,
    difficulty: u8,
    current: Piece,
    game_over: bool,
    winner: Option<Piece>,
    message: String,
    ai_move_queued: bool, // Changed from pending_ai_move to ai_move_queued
}

impl Connect4App {
    fn default(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let settings = terminal::get_player_settings_input();
        let difficulty = terminal::difficulty_input();
        let board = BitBoard::new(settings.0, settings.1, settings.2);
        Self {
            game_state: board,
            difficulty,
            current: Piece::Player,
            game_over: false,
            winner: None,
            message: String::new(),
            ai_move_queued: false, // Initialize ai_move_queued
        }
    }
}

impl eframe::App for Connect4App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for col in 0..self.game_state.cols {
                    ui.vertical(|ui| {
                        let (response, painter) = ui.allocate_painter(
                            egui::Vec2::new(CELL_SIZE, CELL_SIZE * self.game_state.rows as f32),
                            egui::Sense::click(),
                        );

                        if response.clicked() && self.current == Piece::Player && !self.game_over {
                            if let Some(new_board) = self.game_state.drop_piece(col, Piece::Player) {
                                self.game_state = new_board;
                                if self.game_state.check_win(Piece::Player) {
                                    self.game_over = true;
                                    self.message = "You win!".to_string();
                                } else if self.game_state.is_full() {
                                    self.game_over = true;
                                    self.message = "It's a draw!".to_string();
                                } else {
                                    self.current = Piece::AI;
                                    self.ai_move_queued = true;
                                    ctx.request_repaint(); // Request repaint to defer AI move to next frame
                                }
                            }
                        }

                        for row in 0..self.game_state.rows {
                            let piece = self.game_state.get_piece(row, col);
                            let color = match piece {
                                Piece::Empty => egui::Color32::WHITE,
                                Piece::Player => egui::Color32::RED,
                                Piece::AI => egui::Color32::YELLOW,
                            };

                            // Calculate y position to invert rows (0 starts at the bottom)
                            let y = response.rect.top() 
                                + CELL_SIZE * (self.game_state.rows - 1 - row) as f32 
                                + CELL_SIZE / 2.0;

                            let center = egui::Pos2::new(
                                response.rect.left() + CELL_SIZE / 2.0,
                                y
                            );

                            painter.circle_filled(center, CELL_SIZE / 2.0 - CELL_SPACING, color);
                        }
                    });
                }
            });

            // Process AI move in the next frame after player's move is rendered
            if self.ai_move_queued && self.current == Piece::AI && !self.game_over {
                if let Some(col) = minimax(&self.game_state, self.difficulty, i32::MIN, i32::MAX, true).0 {
                    if let Some(new_board) = self.game_state.drop_piece(col, Piece::AI) {
                        self.game_state = new_board;
                        if self.game_state.check_win(Piece::AI) {
                            self.game_over = true;
                            self.message = "AI wins!".to_string();
                        } else if self.game_state.is_full() {
                            self.game_over = true;
                            self.message = "It's a draw!".to_string();
                        } else {
                            self.current = Piece::Player;
                        }
                    }
                } else {
                    self.game_over = true;
                    self.message = "AI has no valid moves!".to_string();
                }
                self.ai_move_queued = false; // Reset the flag
            }

            if self.game_over {
                ui.label(&self.message);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Connect N",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(Connect4App::default(cc)))
        }),
    )
}