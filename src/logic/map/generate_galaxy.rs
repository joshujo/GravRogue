use std::f64;

use glam::DVec2;
use rand::{self, random_range};
use raylib::color::Color;
use shipyard::World;

use crate::logic::map::bodies::{ Acceleration, Colour, Density, Impulse, Mass, Planet, PlanetData, Player, PlayerAcceleration, Position, Size, Star, Velocity };

const GRAVITATIONAL_CONSTANT: f64 = 0.667;

pub fn generate_galaxy() -> World {
    let mut world = World::new();
    let star = generate_star(&mut world);
    let planet = add_planet(&mut world, &star);
    add_player(&mut world, &planet);
    world
}

fn add_planet(world: &mut World, star: &Star) -> PlanetData {
    let planet_orbit = random_range(10.0..=100.0) * star.size;

    let period = f64::sqrt((planet_orbit * planet_orbit * planet_orbit ) / star.k);

    let velocity = (2.0 * f64::consts::PI * planet_orbit) / period;

    let density = random_range(1.0..=15.0);
    let size = random_range(800.0..12000.0);

    let mass = (4.0/3.0) * (size * size * size) * density * f64::consts::PI;

    let grav_acc = (GRAVITATIONAL_CONSTANT * mass) / (size * size);

    world.add_entity((
        Planet,
        Density(density),
        Size(size),
        Colour(Color::BLUE),
        Velocity(DVec2 { x: velocity, y: 0.0 }),
        Acceleration(DVec2 { x: 0.0, y: 0.0 }),
        Position(DVec2 { x: 0.0, y: planet_orbit }),
        Impulse(DVec2::ZERO),
        Mass(mass)
    ));

    PlanetData {
        grav_acc,
        radius: size,
        orbital_radius: planet_orbit,
        velocity
    }
}

fn add_player(world: &mut World, planet: &PlanetData) {
    let position = DVec2::new(0.0, planet.orbital_radius + 2.0);
    let velocity = DVec2::new(planet.velocity, 0.0);
    let player_acc = planet.grav_acc * 10.0;
    let density = 1.0;
    let size = 1.0;

    let mass = (4.0/3.0) * (size * size * size) * density * f64::consts::PI;

    world.add_entity((
        Density(density),
        Size(size),
        Colour(Color::GREEN),
        Velocity(velocity),
        Acceleration(DVec2::new(0.0, 0.0)),
        Position(position),
        Player,
        PlayerAcceleration(player_acc),
        Impulse(DVec2::ZERO),
        Mass(mass)
    ));
}

fn generate_star(world: &mut World) -> Star {
    let density = random_range(1.0..=30.0);
    let size = random_range(10000.0..=100000.0);

    let mass = (4.0/3.0) * f64::consts::PI * (size * size * size) * density;
    
    let k = (GRAVITATIONAL_CONSTANT * mass) / (4.0 * f64::consts::PI * f64::consts::PI);

    world.add_entity((
        Density(density),
        Size(size),
        Colour(Color::WHITE),
        Velocity(DVec2 { x: 0.0, y: 0.0 }),
        Acceleration(DVec2 { x: 0.0, y: 0.0 }),
        Position(DVec2 { x: 0.0, y: 0.0 })
    ));

    Star {
        size,
        mass,
        k,
    }
}
