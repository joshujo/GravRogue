use glam::DVec2;
use raylib::color::Color;
use shipyard::World;

use crate::logic::map::bodies::{Acceleration, Colour, Density, Planet, Position, Size, Velocity};

pub fn generate_galaxy() -> World {
    let mut world = World::new();
    add_planet(&mut world);
    world
}

fn add_planet(world: &mut World) {
    world.add_entity((Planet, Density(10.0), Size(500.0), Colour(Color::BLUE), Velocity(DVec2 {x: 0.0, y: 0.0}), Acceleration(DVec2 { x: 0.0, y: 0.0 }), Position(DVec2 { x: 0.0, y: 0.0 }) ));
}