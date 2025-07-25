use raylib::{ prelude::RaylibDrawHandle, RaylibHandle };

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct WindowData {
    base_x: i32,
    base_y: i32,

    pub current_width: i32,
    pub current_height: i32,

    pub width_scale: f32,
    pub height_scale: f32,

    pub scale: f32,
}

impl WindowData {
    pub fn new(rl: &mut RaylibHandle) -> WindowData {
        let current_width = rl.get_screen_width() as i32;
        let current_height = rl.get_screen_height() as i32;

        let base_x: i32 = 1920;
        let base_y: i32 = 1080;

        let width_scale = (current_width as f32) / (base_x as f32);
        let height_scale = (current_height as f32) / (base_y as f32);

        let scale = f32::sqrt(f32::powi(width_scale, 2) + f32::powi(height_scale, 2));

        WindowData {
            base_x,
            base_y,
            current_width,
            current_height,
            width_scale,
            height_scale,
            scale,
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self, rl: &mut RaylibDrawHandle) {
        self.current_width = rl.get_screen_width() as i32;
        self.current_height = rl.get_screen_height() as i32;

        self.width_scale = (self.current_width as f32) / (self.base_x as f32);
        self.height_scale = (self.current_height as f32) / (self.base_y as f32);
    }

    pub fn center_x(&self) -> f32 {
        (self.current_width / 2) as f32
    }

    pub fn center_y(&self) -> f32 {
        (self.current_height / 2) as f32
    }
}
