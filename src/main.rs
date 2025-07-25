use std::{sync::Arc, thread};

use arc_swap::ArcSwap;
use grav_rogue::{core::{channel_data::RenderData, channels::Channel}, gui::Gui, logic::logic};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title("GravRogue")
        .vsync()
        .resizable()
        .build();

    let render_data = RenderData::default();
    let render_data = Arc::new(ArcSwap::new(Arc::new(render_data)));
    let render_data_2 = Arc::clone(&render_data);


    let channels = Channel::new();
    let logic_channels = channels.logic_channels();
    let gui_channels = channels.gui_channels();

    rl.set_window_min_size(1920, 1080);
    thread::spawn(move || {
        logic(&logic_channels, &render_data);
    });

    

    let mut gui = Gui::new(&mut rl, &thread, &gui_channels);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        gui.tick(&mut d, &render_data_2);
    }
}
