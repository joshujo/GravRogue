use raylib::{camera::Camera2D, color::Color, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2D}};

pub trait Render: Send + Sync {
    fn draw(&self, rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>);
}

pub struct Circle {
    pub radius: f32,
    pub colour: Color,
    pub position: Vector2
}

impl Render for Circle {
    fn draw(&self, rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>) {
        rl.draw_circle(self.position.x as i32, self.position.y as i32, self.radius, self.colour);
    }
}