use std::{ sync::Arc, time::Duration };

use arc_swap::ArcSwap;
use raylib::math::Vector2;
use rayon::iter::{ IntoParallelRefIterator, ParallelIterator };
use shipyard::{ Get, IntoIter, View, World };

use crate::{
    core::{
        channel_data::{ Output, PlayerData, RenderData, StartGame },
        channels::LogicChannels,
        render_data::{ Circle, PlanetRender, Render },
    },
    logic::map::{
        bodies::{ CameraAngle, Colour, Planet, Player, Position, Size },
        generate_galaxy,
    },
};
mod map;
use crate::core::INPUT_STATE;

pub fn logic(logic_channels: &LogicChannels, render_data: &ArcSwap<RenderData>) {
    let mut time;

    loop {
        let _ = logic_channels.main_menu_r.recv().unwrap();
        println!("Logic thread received start game");
        logic_channels.start_game_s.send(StartGame::StartGame).unwrap();
        let mut world = generate_galaxy::generate_galaxy();
        time = std::time::Instant::now();
        loop {
            let input = INPUT_STATE.load();
            if input.restart {
                logic_channels.output_s.send(Output::EndGame).unwrap();
                break;
            }
            let now = std::time::Instant::now();
            let dt = now.duration_since(time).as_secs_f64();
            time = now;
            map::map(&mut world, &dt, logic_channels);
            render(&world, render_data);
            if dt < 0.001 {
                std::thread::sleep(Duration::from_secs_f64(0.001 - dt));
            }
        }
    }
}

fn render(world: &World, render: &ArcSwap<RenderData>) {
    let (pos, size, colour) = world
        .borrow::<(View<Position>, View<Size>, View<Colour>)>()
        .unwrap();

    let (position, player, camera) = world
        .borrow::<(View<Position>, View<Player>, View<CameraAngle>)>()
        .unwrap();

    let (player_position, _, camera) = (&position, &player, &camera).iter().next().unwrap();

    let player = PlayerData {
        position: Vector2::new(player_position.0.x as f32, player_position.0.y as f32),
        camera_angle: camera.0,
    };

    let entities: Vec<_> = (&pos, &size, &colour)
        .iter()
        .with_id()
        .map(|(id,(pos, size, colour))| { (id, pos.0, size.0, colour.0) })
        .collect();

    let data: Vec<Box<dyn Render + Sync + Send>> = entities
        .par_iter()
        .map(|(id, pos, size, colour)| {
            let data: Box<dyn Render + Send + Sync>;
            
            let planet = world.borrow::<View<Planet>>().unwrap();
            if let Ok(_) = planet.get(*id) {
                data = Box::new(PlanetRender {
                radius: *size as f32,
                colour: *colour,
                position: Vector2::new(pos.x as f32, pos.y as f32),
                id: *id
            }) as Box<dyn Render + Send + Sync>;
            } else {
                data = Box::new(Circle {
                radius: *size as f32,
                colour: *colour,
                position: Vector2::new(pos.x as f32, pos.y as f32),
            });
            };
            data as Box<dyn Render + Send + Sync>

            
        })
        .collect();


    let render_data = RenderData {
        data,
        player,
    };

    render.swap(Arc::new(render_data));
}
