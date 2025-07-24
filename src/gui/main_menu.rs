use crate::gui::Gui;

pub(in crate::gui) fn main_menu(data: &mut Gui) {
    let ui = data.imgui.new_frame();

    {
        ui.window("Main Menu")
            .position([0.0, 0.0], imgui::Condition::Always)
            .build(|| {
                ui.text("Main menu");
            });
    }
} 