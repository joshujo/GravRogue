use raylib::{camera::Camera2D, color::Color, math::Vector2, prelude::{RaylibDraw, RaylibMode2D}};

pub trait Render {
    fn draw(&self, rl: &mut RaylibMode2D<Camera2D>);
}

pub struct Circle {
    radius: f32,
    colour: Color,
    position: Vector2
}

impl Render for Circle {
    fn draw(&self, rl: &mut RaylibMode2D<Camera2D>) {
        rl.draw_circle(self.position.x as i32, self.position.y as i32, self.radius, self.colour);
    }
}