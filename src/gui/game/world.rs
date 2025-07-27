use arc_swap::ArcSwap;
use glam::DVec2;
use raylib::{camera::{self, Camera2D}, prelude::{RaylibDrawHandle, RaylibMode2DExt}};
use raylib::consts::KeyboardKey::*;

use crate::{core::channel_data::{Input, RenderData}, gui::Gui};

pub fn world(gui: &mut Gui, rl: &mut RaylibDrawHandle, render_data: &ArcSwap<RenderData>) {
    let mut d: raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>> = rl.begin_mode2D(gui.camera);

    let data = &render_data.load();

    gui.camera.target = data.player.position;

    data.data.iter()
        .for_each(|object| {
            object.as_ref().draw(&mut d);
        });

    {
        let mut player_acc = DVec2::ZERO;

        if d.is_key_down(KEY_W) {
            player_acc.y += 1.0
        }
        if d.is_key_down(KEY_S) {
            player_acc.y -= 1.0
        }
        if d.is_key_down(KEY_A) {
            player_acc.x -= 1.0
        }
        if d.is_key_down(KEY_D) {
            player_acc.x += 1.0
        }

        if player_acc != DVec2::ZERO {
            gui.channels.input_s.send(Input::Acceleration(player_acc)).unwrap();
        }
    }


}