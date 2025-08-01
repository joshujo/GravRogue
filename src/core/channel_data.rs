use glam::DVec2;
use raylib::math::Vector2;

use crate::core::render_data::Render;

pub enum MainMenu {
    StartGame
}

pub enum StartGame {
    StartGame
}

pub enum Input {
    Acceleration(DVec2),
    FireProjectile {
        angle: f64,
    }
}

pub enum Output {
    EndGame
}

#[derive(Default)]
pub struct RenderData {
    pub data: Vec<Box<dyn Render + Send + Sync>>,
    pub player: PlayerData
}

#[derive(Default)]
pub struct PlayerData {
    pub position: Vector2,
    pub camera_angle: f32
}

