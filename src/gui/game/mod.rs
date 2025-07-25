use arc_swap::ArcSwap;
use raylib::{math::Vector2, prelude::{Color, RaylibDraw, RaylibDrawHandle}};
mod world;

use crate::{core::channel_data::RenderData, gui::{game::world::world, Gui}};
use super::fonts::Font::*;

pub fn game(gui: &mut Gui, rl: &mut RaylibDrawHandle, render_data: &ArcSwap<RenderData>) {
    gui.camera.offset = Vector2::new(gui.window_data.center_x(), gui.window_data.center_y());

    {
        rl.draw_text("In game", gui.window_data.center_x() as i32, gui.window_data.center_y() as i32, 30, Color::WHITE);
        world(gui, rl, render_data);
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