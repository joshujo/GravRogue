use arc_swap::ArcSwap;

use raylib::{ math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2DExt}};

use crate::{core::channel_data::{Input, RenderData}, gui::{window_data, Gui, ImguiCache}};

pub fn world(gui: &mut Gui, rl: &mut RaylibDrawHandle, render_data: &ArcSwap<RenderData>) {
    let data = &render_data.load();
    gui.camera.rotation = data.player.camera_angle.to_degrees();
    rl.draw_text(&gui.camera.rotation.to_string(), 500, 0, 50, raylib::color::Color::WHITE);
    let mut d: raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>> = rl.begin_mode2D(gui.camera);

    let buffer = DebugImguiBuffer {
        player_position: data.player.position * 100.0
    };

    gui.imgui_cache.insert(crate::core::ImguiCacheBuffer::Debug, Box::new(buffer));
    

    data.data.iter()
        .for_each(|object| {
            object.as_ref().draw(&mut d, &data.player.position);
        });

    {
        

}

pub struct DebugImguiBuffer {
    pub player_position: Vector2
}

impl ImguiCache for DebugImguiBuffer  {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}}

