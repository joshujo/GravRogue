use crate::{core::channel_data::MainMenu, gui::Gui};
use super::fonts::Font::*;

pub(in crate::gui) fn main_menu(data: &mut Gui) {
    let ui = data.imgui.new_frame();
    let mut new_game = false;

    {
        let ui_width = 1000.0 * data.window_data.width_scale;
        let ui_height = 500.0 * data.window_data.height_scale;

        let title_text = "Main menu";
        let text_scale = data.window_data.scale;

        let center_width = ui_width / 2.0;
        let center_height = ui_height / 2.0;

        ui.window("Main Menu")
            .size([ui_width, ui_height], imgui::Condition::Always)
            .position([data.window_data.center_x() - ui_width / 2.0, data.window_data.center_y() - ui_height / 2.0], imgui::Condition::Always)
            .movable(false)
            .collapsible(false)
            .no_decoration()
            .build(|| {
                ui.set_window_font_scale(text_scale);
                let font = ui.push_font(*data.fonts.get(&ChakraPetchTitle).unwrap());
                let title_size = ui.calc_text_size(title_text);
                let cursor_pos = ui.cursor_pos();
                ui.set_cursor_pos([center_width - title_size[0] / 2.0, cursor_pos[1]]);
                ui.text(title_text);
                font.pop();
                let font = ui.push_font(*data.fonts.get(&ChakraPetchMedium).unwrap());
                let button_size = ui.calc_text_size("New Game");
                ui.set_cursor_pos([center_width - button_size[0] / 2.0, center_height - button_size[1] / 2.0]);
                new_game = ui.button("New Game");
                font.pop();
            });
    }

    if new_game {
        data.channels.main_menu_s.send(MainMenu::StartGame).unwrap();
    }
} 