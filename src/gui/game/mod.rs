use raylib::prelude::{RaylibDraw, RaylibDrawHandle, Color};
mod world;

use crate::gui::Gui;
use super::fonts::Font::*;

pub fn game(gui: &mut Gui, rl: &mut RaylibDrawHandle) {


    {
        rl.draw_text("In game", gui.window_data.center_x() as i32, gui.window_data.center_y() as i32, 30, Color::WHITE);
    }

    {
        let ui = gui.imgui.new_frame();

        ui.window("In game")
            .size([200.0, 50.0], imgui::Condition::Always)
            .position([0.0, 0.0], imgui::Condition::Always)
            .no_decoration()
            .resizable(false)
            .no_inputs()
            .draw_background(false)
            .collapsible(false)
            .build(|| {
                let font = ui.push_font(*gui.fonts.get(&ChakraPetchBold).unwrap());
                ui.text("In game");
                font.pop();
            });
    }
}