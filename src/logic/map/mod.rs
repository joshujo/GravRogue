use std::f64;

use glam::DVec2;
use shipyard::{world::World, IntoIter, View, ViewMut};
use rayon::{self, iter::{IntoParallelRefMutIterator, ParallelIterator}};

use crate::logic::map::bodies::{Acceleration, Density, Position, Size, Velocity};

pub mod bodies;
pub mod generate_galaxy;

pub fn map(world: &mut World, dt: &f64) {
    world.run(gravity_force);
    world.run(|position: ViewMut<Position>, velocity: ViewMut<Velocity>, acceleration: ViewMut<Acceleration>| {
        apply_forces(position, velocity, acceleration, dt);
    });
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
        (pos.0, mass, acc)
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

        acc_a.0 = acc
    });
}

fn apply_forces(mut position: ViewMut<Position>, mut velocity: ViewMut<Velocity>, mut acceleration: ViewMut<Acceleration>, dt: &f64) {
    let mut bodies: Vec<_> = (&mut position, &mut velocity, &mut acceleration)
        .iter()
        .collect();

    bodies.par_iter_mut()
        .for_each(|(pos, vel, acc)| {
            vel.0 += acc.0 * dt;
            pos.0 += vel.0 * dt;
            acc.0 = DVec2::ZERO
        });
}