use std::collections::HashMap;

use crate::{core::{channels::GuiChannels, GameState, ImguiCacheBuffer}, gui::{fonts::{fonts, Font}, main_menu::main_menu, window_data::WindowData}};
use imgui::{self, Context, FontId, FontSource};
use raylib::{prelude::RaylibDrawHandle, RaylibHandle, RaylibThread};
use raylib_imgui_rs::{self, Renderer};
mod main_menu;
mod fonts;
mod window_data;

#[allow(dead_code)]
pub struct Gui<'a> {
    state: GameState,
    channels: &'a GuiChannels,
    imgui: Context,
    fonts: HashMap<Font, FontId>,
    renderer: Renderer,
    imgui_cache: HashMap<ImguiCacheBuffer, Box<dyn ImguiCache>>,
    window_data: WindowData
}

pub trait ImguiCache {}

impl <'a>Gui<'a> {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, channels: &'a GuiChannels) -> Self {
        let mut imgui = Context::create();
        let fonts = fonts(&mut imgui);
        imgui.fonts().add_font(&[FontSource::DefaultFontData { config: None }]);
        let renderer = Renderer::create(&mut imgui,rl, &thread);
        let window_data = WindowData::new(rl);

        Gui { 
            state: GameState::MainMenu,
            channels: &channels,
            imgui,
            fonts,
            renderer,
            imgui_cache: HashMap::new(),
            window_data
        }
    }

    pub fn tick(&mut self, rl: &mut RaylibDrawHandle) {
        self.window_data.update(rl);
        self.renderer.update(&mut self.imgui, rl);

        match self.state {
            GameState::MainMenu => main_menu(self),
        }

        self.renderer.render(&mut self.imgui, rl);
    }
}