use std::collections::HashMap;

use crate::{core::{GameState, ImguiCacheBuffer}, gui::{fonts::fonts, main_menu::main_menu}};
use imgui::{self, Context, FontSource};
use raylib::{prelude::RaylibDrawHandle, RaylibHandle, RaylibThread};
use raylib_imgui_rs::{self, Renderer};
mod main_menu;
mod fonts;

#[allow(dead_code)]
pub struct Gui {
    state: GameState,
    imgui: Context,
    renderer: Renderer,
    imgui_cache: HashMap<ImguiCacheBuffer, Box<dyn ImguiCache>>,
}

pub trait ImguiCache {}

impl Gui {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut imgui = Context::create();
        fonts(&mut imgui);
        imgui.fonts().add_font(&[FontSource::DefaultFontData { config: None }]);
        let renderer = Renderer::create(&mut imgui,rl, &thread);
        

        Gui { 
            state: GameState::MainMenu,
            imgui,
            renderer,
            imgui_cache: HashMap::new()
        }
    }

    pub fn tick(&mut self, rl: &mut RaylibDrawHandle) {
        self.renderer.update(&mut self.imgui, rl);

        match self.state {
            GameState::MainMenu => main_menu(self),
        }

        self.renderer.render(&mut self.imgui, rl);
    }
}