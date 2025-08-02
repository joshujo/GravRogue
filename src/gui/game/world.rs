use arc_swap::ArcSwap;

use raylib::{ prelude::{ RaylibDraw, RaylibDrawHandle, RaylibMode2DExt } };

use crate::{ core::channel_data::RenderData, gui::Gui };

pub fn world(gui: &mut Gui, rl: &mut RaylibDrawHandle, render_data: &ArcSwap<RenderData>) {
    let data = &render_data.load();
    gui.camera.rotation = data.player.camera_angle.to_degrees();
    rl.draw_text(&gui.camera.rotation.to_string(), 500, 0, 50, raylib::color::Color::WHITE);
    let mut d: raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>> = rl.begin_mode2D(
        gui.camera
    );

    data.data.iter().for_each(|object| {
        if gui.camera.zoom >= 0.00001 {
            object.as_ref().draw(&mut d, &data.player.position, gui);
        } else {
            object.as_ref().draw_map(&mut d, &data.player.position);
        }
    });

    {}
}
