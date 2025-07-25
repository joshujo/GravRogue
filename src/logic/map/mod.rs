use std::f64;

use glam::DVec2;
use shipyard::{world::World, IntoIter, View, ViewMut};
use rayon::{self, iter::{IntoParallelRefMutIterator, ParallelIterator}};

use crate::logic::map::bodies::{Acceleration, Density, Position, Size, Velocity};

pub mod bodies;
pub mod generate_galaxy;

pub fn map(world: &mut World) {
    world.run(gravity_force);
    world.run(apply_forces);
}

fn gravity_force(mut acceleration: ViewMut<Acceleration>, density: View<Density>, size: View<Size>, position: View<Position>) {
    const GRAVITATIONAL_CONSTANT: f64 = 0.667;

    let bodies: Vec<_> = (&position, &density, &size)
        .iter()
        .map(|(pos, des, size)| {
            let mass = des.0 * f64::powi(size.0, 2) * f64::consts::PI;
            (pos.0, mass)
        }).collect();

    let mut accel_bodies: Vec<_> = (&mut acceleration, &density, &position, &size).iter().map(|(acc, dens, pos, size)| {
        let mass = dens.0 * f64::powi(size.0, 2) * f64::consts::PI;
        (pos.0, mass, acc.0)
    }).collect();

    accel_bodies.par_iter_mut()
    .for_each(|(pos_a, mass_a, acc_a)| {
        let mut acc = DVec2::ZERO;

        for (pos_b, mass_b) in &bodies {
            if pos_a == pos_b {
                continue;
            }

            let radius = DVec2::distance(*pos_a, *pos_b);
            let direction = (*pos_b - *pos_a).normalize();

            let force_magnitude = (GRAVITATIONAL_CONSTANT * *mass_b * *mass_a) / f64::powi(radius, 2);

            acc += direction * force_magnitude / *mass_a;
        }

        *acc_a = acc
    });
}

fn apply_forces(mut position: ViewMut<Position>, mut velocity: ViewMut<Velocity>, acceleration: View<Acceleration>) {
    let mut bodies: Vec<_> = (&mut position, &mut velocity, &acceleration)
        .iter()
        .map(|(pos, vel, acc)| {
            (pos.0, vel.0, acc.0)
        }).collect();

    bodies.par_iter_mut()
        .for_each(|(pos, vel, acc)| {
            *vel += *acc;
            *pos += *vel;
            *acc = DVec2::ZERO
        });
}