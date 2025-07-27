use arc_swap::ArcSwap;
use crossbeam::channel::Receiver;
use raylib::{ math::Vector2, prelude::{ Color, RaylibDraw, RaylibDrawHandle } };
mod world;
use raylib::consts::KeyboardKey::*;

use crate::{
    core::{channel_data::RenderData, GameState},
    gui::{ game::world::world, Gui },
};
use super::fonts::Font::*;

pub fn game(gui: &mut Gui, rl: &mut RaylibDrawHandle, render_data: &ArcSwap<RenderData>) {
    gui.camera.offset = Vector2::new(gui.window_data.center_x(), gui.window_data.center_y());
    {
        world(gui, rl, render_data);
    }

    if let Ok(r) = gui.channels.output_r.try_recv() {
        match r {
            crate::core::channel_data::Output::EndGame => {
                gui.state = GameState::MainMenu
            }
        }
    }

    {
        let mut scroll: Option<Scroll> = None;
        if rl.is_key_down(KEY_Q) {
            scroll = Some(Scroll::Up);
        } else if rl.is_key_down(KEY_E) {
            scroll = Some(Scroll::Down);
        }

        if let Some(scroll) = scroll {
            match scroll {
                Scroll::Up => {
                        gui.camera.zoom *= 1.0 + 1.0 * 0.05;
                        gui.camera.zoom = gui.camera.zoom.clamp(0.0000001, 1.0);
                },
                Scroll::Down => {
                        gui.camera.zoom *= 1.0 - 1.0 * 0.05;
                        gui.camera.zoom = gui.camera.zoom.clamp(0.0000001, 1.0);
                }
            }
        }
    }

    {
        let ui = gui.imgui.new_frame();
        ui.window("In game")
            .size([500.0, 500.0], imgui::Condition::Always)
            .position([0.0, 0.0], imgui::Condition::Always)
            .no_decoration()
            .resizable(false)
            .no_inputs()
            .draw_background(false)
            .collapsible(false)
            .build(|| {
                let font = ui.push_font(*gui.fonts.get(&ChakraPetchBold).unwrap());
                ui.text("In game");
                ui.text(format!("{:?}", gui.camera.target));
                font.pop();
            });
    }
}

enum Scroll {
    Up,
    Down,
}
