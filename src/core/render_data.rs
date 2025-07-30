use raylib::{color::Color, math::Vector2, prelude::{RaylibDraw, RaylibDrawHandle}};

pub trait Render: Send + Sync {
    fn draw(&self, rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>, player_position: &Vector2);
}

pub struct Circle {
    pub radius: f32,
    pub colour: Color,
    pub position: Vector2
}

impl Render for Circle {
    fn draw(&self, rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>, player_position: &Vector2) {
        //rl.draw_circle((self.position.x * 100.0) as i32, (self.position.y * 100.0) as i32, self.radius * 100.0, self.colour);
        let x = (self.position.x - player_position.x) * 100.0;
        let y = - (self.position.y - player_position.y) * 100.0;

        let center = Vector2::new(x, y);
        rl.draw_circle_sector(center, self.radius * 100.0, 0.0, 360.0, 2048, self.colour);
    }
}