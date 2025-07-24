use std::thread;

use grav_rogue::{gui::Gui, logic::logic};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title("GravRogue")
        .vsync()
        .build();

    thread::spawn(|| {
        logic();
    });

    let mut gui = Gui::new(&mut rl, &thread);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        gui.tick(&mut d);
    }
}
