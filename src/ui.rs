use crate::minimax::minimax;
use crate::bitboard::{BitBoard, Piece};
use eframe::egui;
use eframe::egui::{Visuals};
use crate::terminal::{first_player_input, get_player_settings_input, difficulty_input};

const CELL_SIZE: f32 = 50.0;
const CELL_SPACING: f32 = 5.0;

pub struct Connect4App {
    game_state: BitBoard,
    difficulty: u8,
    current: Piece,
    game_over: bool,
    message: String,
    ai_move_queued: bool,
}

impl Connect4App {
    pub fn default(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        let settings = get_player_settings_input();
        let difficulty = difficulty_input();
        let board = BitBoard::new(settings.0, settings.1, settings.2);
        let player_starts = first_player_input();
        let current = match player_starts {
            true => Piece::Player,
            false => Piece::AI,
        };

        Self {
            game_state: board,
            difficulty,
            current:  current,
            game_over: false,
            message: String::new(),
            ai_move_queued: false,
        }
    }
}

impl eframe::App for Connect4App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_visuals(Visuals::dark());
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
                                    self.message = "Zmagal si!".to_string();
                                } else if self.game_state.is_full() {
                                    self.game_over = true;
                                    self.message = "Izenačeno!".to_string();
                                } else {
                                    self.current = Piece::AI;
                                    self.ai_move_queued = true;
                                    ctx.request_repaint();
                                }
                            }
                        }

                        for row in 0..self.game_state.rows {
                            let piece = self.game_state.get_piece(row, col);
                            let color = match piece {
                                Piece::Empty => egui::Color32::GRAY,
                                Piece::Player => egui::Color32::RED,
                                Piece::AI => egui::Color32::YELLOW,
                            };

                            // izračunaj y pozicijo, da obrneš vrstice (0 je na dnu)
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

            // procesiraj AI potezo v naslednji sličici, potem ko je igralčeva poteza narisana
            if self.ai_move_queued && self.current == Piece::AI && !self.game_over {
                if let Some(col) = minimax(&self.game_state, self.difficulty, i32::MIN, i32::MAX, true).0 {
                    if let Some(new_board) = self.game_state.drop_piece(col, Piece::AI) {
                        self.game_state = new_board;
                        if self.game_state.check_win(Piece::AI) {
                            self.game_over = true;
                            self.message = "AI zmaga!".to_string();
                        } else if self.game_state.is_full() {
                            self.game_over = true;
                            self.message = "Izenačeno!".to_string();
                        } else {
                            self.current = Piece::Player;
                        }
                    }
                } else {
                    self.game_over = true;
                    self.message = "Ni veljavnih potez za AI!".to_string();
                }
                self.ai_move_queued = false;
            }

            if self.game_over {
                ui.label(&self.message);
            }
        });
    }
}