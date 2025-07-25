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

pub struct RenderData {
    data: Vec<Box<dyn Render>>
}
