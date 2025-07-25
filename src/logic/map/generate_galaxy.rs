use glam::DVec2;
use raylib::color::Color;
use shipyard::World;

use crate::logic::map::bodies::{ Acceleration, Colour, Density, Planet, Position, Size, Velocity };

pub fn generate_galaxy() -> World {
    let mut world = World::new();
    add_planet(&mut world);
    world
}

fn add_planet(world: &mut World) {
    world.add_entity((
        Planet,
        Density(100.0),
        Size(100.0),
        Colour(Color::BLUE),
        Velocity(DVec2 { x: 0.0, y: 0.0 }),
        Acceleration(DVec2 { x: 0.0, y: 0.0 }),
        Position(DVec2 { x: 300.0, y: -200.0 }),
    ));
    world.add_entity((
        Planet,
        Density(50.0),
        Size(50.0),
        Colour(Color::BLUE),
        Velocity(DVec2 { x: 0.0, y: 0.0 }),
        Acceleration(DVec2 { x: 0.0, y: 0.0 }),
        Position(DVec2 { x: -200.0, y: 300.0 }),
    ));
    world.add_entity((
        Planet,
        Density(80.0),
        Size(70.0),
        Colour(Color::GREEN),
        Velocity(DVec2 { x: 0.0, y: 1.0 }),
        Acceleration(DVec2::ZERO),
        Position(DVec2 { x: -500.0, y: -300.0 }),
    ));

    world.add_entity((
        Planet,
        Density(20.0),
        Size(30.0),
        Colour(Color::RED),
        Velocity(DVec2 { x: 1.5, y: -1.0 }),
        Acceleration(DVec2::ZERO),
        Position(DVec2 { x: 400.0, y: 250.0 }),
    ));

    world.add_entity((
        Planet,
        Density(60.0),
        Size(40.0),
        Colour(Color::YELLOW),
        Velocity(DVec2 { x: -0.5, y: 0.5 }),
        Acceleration(DVec2::ZERO),
        Position(DVec2 { x: 0.0, y: 0.0 }),
    ));
}
