use raylib::color::Color;
use shipyard::{self, Component};
use glam::{f64, DVec2};

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct Density(pub f64);

#[derive(Component)]
pub struct Size(pub f64);

#[derive(Component)]
pub struct Colour(pub Color);

#[derive(Component)]
pub struct Velocity(pub DVec2);

#[derive(Component)]
pub struct Acceleration(pub DVec2);

#[derive(Component)]
pub struct Position(pub DVec2);
