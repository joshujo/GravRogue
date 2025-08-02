use image::load_from_memory_with_format;
use raylib::{ color::Color, ffi::PixelFormat, math::Vector2, prelude::{ RaylibDraw, RaylibDrawHandle } };
use shipyard::EntityId;

use crate::{ core::texture_gen::generate_planet_texture, gui::Gui };

pub trait Render: Send + Sync {
    fn draw(
        &self,
        rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>,
        player_position: &Vector2,
        gui: &mut Gui
    );
    fn draw_map(
        &self,
        rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>,
        player_position: &Vector2
    );
}

pub trait RenderPlanet: Send + Sync {
    fn draw(
        &self,
        rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>,
        player_position: &Vector2,
        gui: &mut Gui
    );
    fn draw_map(
        &self,
        rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>,
        player_position: &Vector2
    );
}

pub struct Circle {
    pub radius: f32,
    pub colour: Color,
    pub position: Vector2,
}

pub struct PlanetRender {
    pub radius: f32,
    pub colour: Color,
    pub position: Vector2,
    pub id: EntityId,
}

impl Render for Circle {
    fn draw(
        &self,
        rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>,
        player_position: &Vector2,
        _gui: &mut Gui
    ) {
        //rl.draw_circle((self.position.x * 100.0) as i32, (self.position.y * 100.0) as i32, self.radius * 100.0, self.colour);
        let x = (self.position.x - player_position.x) * 100.0;
        let y = -(self.position.y - player_position.y) * 100.0;

        let center = Vector2::new(x, y);
        rl.draw_circle_sector(center, self.radius * 100.0, 0.0, 360.0, 2048, self.colour);
    }

    fn draw_map(
        &self,
        rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>,
        player_position: &Vector2
    ) {
        let x = (self.position.x - player_position.x) * 100.0;
        let y = -(self.position.y - player_position.y) * 100.0;

        let center = Vector2::new(x, y).scale_by(100.0);
        rl.draw_circle_sector(center, self.radius * 10000.0, 0.0, 360.0, 2048, self.colour);
    }
}

impl Render for PlanetRender {
    fn draw(
        &self,
        rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>,
        player_position: &Vector2,
        gui: &mut Gui
    ) {
        let x = (self.position.x - player_position.x) * 100.0;
        let y = -(self.position.y - player_position.y) * 100.0;

        let center = Vector2::new(x, y);
        rl.draw_circle_sector(center, self.radius * 100.0, 0.0, 360.0, 2048, self.colour);
        match gui.planet_textures.get(&self.id) {
            Some(texture) => {
                let scale = (self.radius * 100.0) / 128.0;
                let scaled_size = scale * 256.0;
                let texture_center = Vector2 {
                    x: center.x - scaled_size / 2.0,
                    y: center.y - scaled_size / 2.0
                };
                rl.draw_texture_ex(texture, texture_center, 0.0, scale, Color::WHITE);
            }
            None => {
                let pixels = generate_planet_texture();
                println!("PNG byte length: {}", pixels.len());
                std::fs::write("planet_test.png", &pixels).unwrap();
                std::fs::write("debug_output.txt", &pixels).unwrap();
                
                let image = raylib::core::texture::Image::load_image_from_mem(".bmp", &pixels).unwrap();

                let texture = rl.load_texture_from_image(gui.thread, &image).unwrap();

                gui.planet_textures.insert(self.id, texture); 
            }
        }
        
    }

    fn draw_map(
        &self,
        rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>,
        player_position: &Vector2
    ) {
        let x = (self.position.x - player_position.x) * 100.0;
        let y = -(self.position.y - player_position.y) * 100.0;

        let center = Vector2::new(x, y).scale_by(100.0);
        rl.draw_circle_sector(center, self.radius * 10000.0, 0.0, 360.0, 2048, self.colour);
    }
}

impl RenderPlanet for PlanetRender {
    fn draw(
        &self,
        rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>,
        player_position: &Vector2,
        gui: &mut Gui
    ) {
        let x = (self.position.x - player_position.x) * 100.0;
        let y = -(self.position.y - player_position.y) * 100.0;

        let center = Vector2::new(x, y);
        match gui.planet_textures.get(&self.id) {
            Some(texture) => {
                rl.draw_texture(texture, x as i32, y as i32, Color::WHITE);
            }
            None => {
                let pixels = generate_planet_texture();
                let image = raylib::prelude::Image::load_image_from_mem("png", &pixels).unwrap();
                let texture = rl.load_texture_from_image(gui.thread, &image).unwrap();
                gui.planet_textures.insert(self.id, texture);
                rl.draw_texture(
                    gui.planet_textures.get(&self.id).unwrap(),
                    x as i32,
                    y as i32,
                    Color::WHITE
                );
            }
        }
        rl.draw_circle_sector(center, self.radius * 100.0, 0.0, 360.0, 2048, self.colour);
    }

    fn draw_map(
        &self,
        rl: &mut raylib::prelude::RaylibMode2D<'_, RaylibDrawHandle<'_>>,
        player_position: &Vector2
    ) {
        let x = (self.position.x - player_position.x) * 100.0;
        let y = -(self.position.y - player_position.y) * 100.0;

        let center = Vector2::new(x, y).scale_by(100.0);
        rl.draw_circle_sector(center, self.radius * 10000.0, 0.0, 360.0, 2048, self.colour);
    }
}
