use std::sync::Arc;

use arc_swap::ArcSwap;
use raylib::math::Vector2;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shipyard::{IntoIter, View, World};

use crate::{core::{channel_data::{RenderData, StartGame}, channels::LogicChannels, render_data::{Circle, Render}}, logic::map::{bodies::{Colour, Position, Size}, generate_galaxy}};
mod map;

pub fn logic(logic_channels: &LogicChannels, render_data: &ArcSwap<RenderData>) {
    let mut time;
    loop {
        let _ = logic_channels.main_menu_r.recv().unwrap();
        println!("Logic thread received start game");
        logic_channels.start_game_s.send(StartGame::StartGame).unwrap();
        let mut world = generate_galaxy::generate_galaxy();
        time = std::time::Instant::now();
        loop {
            let now = std::time::Instant::now();
            let dt = now.duration_since(time).as_secs_f64();
            time = now;
            map::map(&mut world, &dt);
            render(&world, render_data);
        }
    }
}

fn render(world: &World, render: &ArcSwap<RenderData>) {
    let (pos, size, colour) = world.borrow::<(View<Position>, View<Size>, View<Colour>)>().unwrap();

    let entities: Vec<_> = (&pos, &size, &colour)
        .iter().map(|(pos, size, colour)| {
            (pos.0, size.0, colour.0)
        }).collect();

    let data: Vec<Box<dyn Render + Sync + Send>> = entities.par_iter()
        .map(|(pos, size, colour)| {
            let data = Box::new(Circle {
                radius: *size as f32,
                colour: *colour,
                position: Vector2::new(pos.x as f32, pos.y as f32)
            });
            data as Box<dyn Render + Send + Sync>
        }).collect();
    
    let render_data = RenderData {
        data
    };

    render.swap(Arc::new(render_data));
}