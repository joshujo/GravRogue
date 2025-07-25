use arc_swap::ArcSwap;
use raylib::{camera::{self, Camera2D}, prelude::{RaylibDrawHandle, RaylibMode2DExt}};

use crate::{core::channel_data::RenderData, gui::Gui};

pub fn world(gui: &mut Gui, rl: &mut RaylibDrawHandle, render_data: &ArcSwap<RenderData>) {
    let mut d: raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>> = rl.begin_mode2D(gui.camera);

    let data = &render_data.load().data;

    data.iter()
        .for_each(|object| {
            object.as_ref().draw(&mut d);
        });
}