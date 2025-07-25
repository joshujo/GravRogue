use crate::core::render_data::Render;

pub enum MainMenu {
    StartGame
}

pub enum StartGame {
    StartGame
}

pub enum Input {
    
}

pub enum Output {
    
}

#[derive(Default)]
pub struct RenderData {
    pub data: Vec<Box<dyn Render + Send + Sync>>
}
