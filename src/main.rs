mod bitboard;
mod terminal;
mod minimax;
mod ui;

use ui::Connect4App;
use eframe::egui;
use terminal::{game_mode_settings_input, main_loop_terminal, Mode};


// glavna funkcija, ki se izvede ob zagonu programa
fn main() -> eframe::Result<()> {
    // določi vrsto igre (terminal ali novo okno)
    let mode = game_mode_settings_input();

    if mode == Mode::Ui {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([600.0, 400.0]),
            ..Default::default()
        };

        eframe::run_native(
            "N v vrsto",
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
