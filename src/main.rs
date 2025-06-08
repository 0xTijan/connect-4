mod bitboard;
mod terminal;
mod minimax;
mod ui;

use ui::Connect4App;
use eframe::egui;
use terminal::{game_mode_settings_input, main_loop_terminal, Mode};

fn main() -> eframe::Result<()> {
    let mode = game_mode_settings_input();

    if mode == Mode::Ui {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([600.0, 400.0]),
            ..Default::default()
        };

        eframe::run_native(
            "Connect N",
            options,
            Box::new(|cc| {
                Ok(Box::new(Connect4App::default(cc)))
            }),
        )
    } else {
        main_loop_terminal();
        Ok(())
    }
}
