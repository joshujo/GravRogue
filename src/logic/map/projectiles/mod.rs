use std::f64;

use glam::DVec2;
use shipyard::World;

use raylib::color::Color;

use crate::logic::map::bodies::{Acceleration, Colour, Density, Mass, Position, Projectile, Velocity};

pub fn projectile_fire(
    world: &mut World,
    position: DVec2,
    angle: f64,
    projectile_size: f64,
    density: f64,
    muzzle_velocity: f64,
    initial_velocity: DVec2
) {
    let velocity_x = initial_velocity.x + muzzle_velocity * f64::cos(angle);
    let velocity_y = initial_velocity.y + muzzle_velocity * f64::sin(angle);

    let velocity = DVec2 {
        x: velocity_x,
        y: velocity_y
    };

    let mass = (4.0 / 3.0) * f64::consts::PI * (projectile_size * projectile_size * projectile_size);

    world.add_entity((
        Projectile,
        Velocity(velocity),
        Acceleration::default(),
        Mass(mass),
        Density(density),
        Position(position),
        Colour(Color::YELLOW)
    ));
}