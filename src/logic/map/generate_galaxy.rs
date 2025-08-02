use std::{f32, f64};

use glam::DVec2;
use rand::{self, random_range};
use raylib::color::Color;
use shipyard::World;

use crate::logic::map::bodies::{ Acceleration, CameraAngle, Colour, Density, Impulse, Mass, Planet, PlanetData, Player, PlayerAcceleration, Position, Size, Star, Terrain, TerrainData, Velocity };

const GRAVITATIONAL_CONSTANT: f64 = 0.000667;

pub fn generate_galaxy() -> World {
    let mut world = World::new();
    let star = generate_star(&mut world);
    let planet = add_planet(&mut world, &star);
    add_player(&mut world, &planet);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    add_planet(&mut world, &star);
    world
}

fn add_planet(world: &mut World, star: &Star) -> PlanetData {
    let planet_orbit = random_range(10.0..=80.0) * star.size;

    let angle: f64 = random_range(0.0..=360.0_f64).to_radians();

    let period = f64::sqrt((planet_orbit * planet_orbit * planet_orbit ) / star.k);

    let velocity = (2.0 * f64::consts::PI * planet_orbit) / period;

    let position = DVec2 { x: planet_orbit * f64::cos(angle), y: planet_orbit * f64::sin(angle) };
    let direction = position.normalize();

    let tangent = DVec2::new(direction.x, -direction.y);
    let velocity = tangent * velocity;


    let density = random_range(1.0..=10.0);
    let size = random_range(1500.0..10000.0);

    let mass = (4.0/3.0) * (size * size * size) * density * f64::consts::PI;

    let grav_acc = (GRAVITATIONAL_CONSTANT * mass) / (size * size);

    world.add_entity((
        Planet,
        Density(density),
        Size(size),
        Colour(Color::BLUE),
        Velocity(velocity),
        Acceleration(DVec2 { x: 0.0, y: 0.0 }),
        Position(position),
        Impulse(DVec2::ZERO),
        Mass(mass),
    ));

    PlanetData {
        grav_acc,
        radius: size,
        velocity,
        mass,
        position: DVec2 { x: planet_orbit * f64::cos(angle), y: planet_orbit * f64::sin(angle) }
    }
}

fn add_player(world: &mut World, planet: &PlanetData) {
    let density = 1.0;
    let size = 2.0;
    let mass = (4.0/3.0) * (size * size * size) * density * f64::consts::PI;

    let player_acc = planet.grav_acc * 1.1;

    let radius = planet.radius * 1.1;

    let velocity = f64::sqrt((GRAVITATIONAL_CONSTANT * planet.mass) / radius);
    let velocity = DVec2::new(planet.velocity.x + velocity, planet.velocity.y);
    let position = DVec2::new(planet.position.x, planet.position.y + radius);

    let player = world.add_entity((
        Density(density),
        Size(size),
        Colour(Color::GREEN),
        Velocity(velocity),
        Acceleration(DVec2::new(0.0, 0.0)),
        Position(position),
        Player,
        PlayerAcceleration(player_acc),
        Impulse(DVec2::ZERO),
        Mass(mass),
    ));

    world.add_component(player, CameraAngle(0.0));
}

fn generate_star(world: &mut World) -> Star {
    let density = random_range(1.0..=20.0);
    let size = random_range(30000.0..=300000.0);

    let mass = (4.0/3.0) * f64::consts::PI * (size * size * size) * density;
    
    let k = (GRAVITATIONAL_CONSTANT * mass) / (4.0 * f64::consts::PI * f64::consts::PI);

    world.add_entity((
        Density(density),
        Size(size),
        Colour(Color::WHITE),
        Velocity(DVec2 { x: 0.0, y: 0.0 }),
        Acceleration(DVec2 { x: 0.0, y: 0.0 }),
        Mass(mass),
        Position(DVec2 { x: 0.0, y: 0.0 }),
        Impulse(DVec2 { x: 0.0, y: 0.0 })
    ));

    Star {
        size,
        k,
    }
}
