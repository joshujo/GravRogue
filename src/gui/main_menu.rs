use crate::gui::Gui;

pub(in crate::gui) fn main_menu(data: &mut Gui) {
    let ui = data.imgui.new_frame();

    {
        let ui_width = 1000.0 * data.window_data.width_scale;
        let ui_height = 500.0 * data.window_data.height_scale;

        let title_text = "Main menu";
        // let text_scale = data.window_data.scale;

        let center_width = ui_width / 2.0;

        ui.window("Main Menu")
            .size([ui_width, ui_height], imgui::Condition::Always)
            .position([data.window_data.center_x() - ui_width / 2.0, data.window_data.center_y() - ui_height / 2.0], imgui::Condition::Always)
            .movable(false)
            .collapsible(false)
            .no_decoration()
            .build(|| {
                let font = ui.push_font(*data.fonts.get(&super::fonts::Font::ChakraPetchTitle).unwrap());
                let title_size = ui.calc_text_size(title_text);
                let cursor_pos = ui.cursor_pos();
                ui.set_cursor_pos([center_width - title_size[0] / 2.0, cursor_pos[1]]);
                ui.text(title_text);
                font.pop();
                
            });
    }
} 