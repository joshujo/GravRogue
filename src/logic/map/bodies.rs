use raylib::color::Color;
use shipyard::{self, Component, Unique};
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

#[derive(Component, Default)]
pub struct Acceleration(pub DVec2);

#[derive(Component)]
pub struct Position(pub DVec2);

#[derive(Component)]
pub struct Mass(pub f64);

#[derive(Component)]
pub struct Terrain(pub Vec<TerrainData>);

pub struct Star {
    pub size: f64,
    pub k: f64,
}

#[derive(Component, Unique)]
pub struct Player;

#[derive(Component, Unique)]
pub struct PlayerAcceleration(pub f64); 

pub struct PlanetData {
    pub grav_acc: f64,
    pub radius: f64,
    pub velocity: DVec2,
    pub mass: f64,
    pub position: DVec2
}

#[derive(Component)]
pub struct Impulse(pub DVec2);

#[derive(Component)]
pub struct CameraAngle(pub f32);

pub struct TerrainData {
    pub angle: f32,
    pub radius: f32
}

#[derive(Component)]
pub struct Projectile;

