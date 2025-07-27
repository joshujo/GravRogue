use std::f64;

use glam::DVec2;
use shipyard::{world::World, IntoIter, View, ViewMut};
use rayon::{self, iter::{IntoParallelRefMutIterator, ParallelIterator}};

use crate::{core::channels::LogicChannels, logic::map::bodies::{Acceleration, Density, Impulse, Mass, Player, PlayerAcceleration, Position, Size, Velocity}};

pub mod bodies;
pub mod generate_galaxy;

use crate::core::INPUT_STATE;

const GRAVITATIONAL_CONSTANT: f64 = 0.00667;

pub fn map(world: &mut World, dt: &f64, channels: &LogicChannels) {
    world.run(gravity_force);

    let input = INPUT_STATE.load();

    let player_acc = {
        let mut acc = DVec2::ZERO;
        if input.up {
            acc.y += 1.0
        }
        if input.down {
            acc.y -= 1.0
        }
        if input.left {
            acc.x -= 1.0 
        }
        if input.right {
            acc.x += 1.0
        }
        acc
    };

    world.run(|acceleration: ViewMut<Acceleration>, player: View<Player>, player_accel: View<PlayerAcceleration>| {
                    player_movement(acceleration, player, player_accel, player_acc);
                });
    world.run(|position: ViewMut<Position>, velocity: ViewMut<Velocity>, acceleration: ViewMut<Acceleration>| {
        apply_forces(position, velocity, acceleration, dt);
    });
    world.run(collision);
    world.run(apply_impulse)
}

fn player_movement(mut acceleration: ViewMut<Acceleration>, player: View<Player>, player_accel: View<PlayerAcceleration>, player_acc: DVec2) {
    (&mut acceleration, &player, &player_accel).iter()
        .for_each(|(acceleration, _, player_accel)| {
            acceleration.0 += player_acc * player_accel.0
        });
}

fn gravity_force(mut acceleration: ViewMut<Acceleration>, density: View<Density>, size: View<Size>, position: View<Position>) {

    let bodies: Vec<_> = (&position, &density, &size)
        .iter()
        .map(|(pos, dens, size)| {
            let mass = dens.0 * (size.0 * size.0 * size.0) * (4.0 / 3.0) * f64::consts::PI;
            (pos.0, mass)
        }).collect();

    let mut accel_bodies: Vec<_> = (&mut acceleration, &density, &position, &size).iter().map(|(acc, dens, pos, size)| {
        let mass = dens.0 * (size.0 * size.0 * size.0) * (4.0 / 3.0) * f64::consts::PI;
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

            let force_magnitude = (GRAVITATIONAL_CONSTANT * *mass_b * *mass_a) / (radius * radius);

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

fn collision(pos: ViewMut<Position>, size: View<Size>, velocity: View<Velocity>, mass: View<Mass>, mut impulse: ViewMut<Impulse>) {
    let bodies: Vec<_> = (&pos, &size, &velocity, &mass, &mut impulse)
        .iter()
        .map(|(pos, size, vel, mass, _)| {
            (pos.0, size.0, vel.0, mass.0)
        })
        .collect();

    let mut impulse_bodies: Vec<_> = (&pos, &size, &velocity, &mass, &mut impulse)
        .par_iter()
        .collect();

    impulse_bodies.par_iter_mut()
        .for_each(|(pos_a, size_a, vel_a, mass_a, impulse)| {

            for (pos_b, size_b, vel_b, mass_b) in &bodies {
                if pos_a.0 == *pos_b {
                    continue;
                }

                let distance = DVec2::distance(pos_a.0, *pos_b);

                if distance > size_a.0 + *size_b {
                    continue;
                }

                let normal = (pos_a.0 - *pos_b).normalize();
                let relative_velocity = vel_a.0 - *vel_b;
                let speed_along_normal = relative_velocity.dot(normal);

                if speed_along_normal >= 0.0 {
                    continue;
                }

                let restitution = 0.01;
                let impulse_magnitude = -(1.0 + restitution) * speed_along_normal / (1.0 / mass_a.0 + 1.0 / *mass_b);


                impulse.0 += normal * impulse_magnitude;
            }

        });
}

fn apply_impulse(mut velocity: ViewMut<Velocity>, mut impulse: ViewMut<Impulse>, mass: View<Mass>) {
    let mut bodies: Vec<_> = (&mut velocity, &mut impulse, &mass).par_iter()
        .collect();

    bodies.par_iter_mut()
        .for_each(|(vel, impulse, mass )| {
            vel.0 += impulse.0 / mass.0;
            impulse.0 = DVec2::ZERO;
        });
}