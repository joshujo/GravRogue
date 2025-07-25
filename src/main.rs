use std::thread;

use grav_rogue::{core::channels::Channel, gui::Gui, logic::logic};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title("GravRogue")
        .vsync()
        .resizable()
        .build();

    let channels = Channel::new();
    let logic_channels = channels.logic_channels();
    let gui_channels = channels.gui_channels();

    rl.set_window_min_size(1920, 1080);
    thread::spawn(move || {
        logic(&logic_channels);
    });

    

    let mut gui = Gui::new(&mut rl, &thread, &gui_channels);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        gui.tick(&mut d);
    }
}
