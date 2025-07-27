use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

pub mod channel_data;
pub mod channels;
pub mod render_data;

pub enum GameState {
    MainMenu,
    Game
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImguiCacheBuffer {
    MainMenu,
    Debug
}

pub static INPUT_STATE: Lazy<ArcSwap<InputState>> = Lazy::new(|| ArcSwap::from_pointee(InputState::default()));

#[derive(Default)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub restart: bool,
}