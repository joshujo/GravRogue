use std::collections::HashMap;

use crate::{core::{channel_data::RenderData, channels::GuiChannels, GameState, ImguiCacheBuffer}, gui::{fonts::{fonts, Font}, game::game, main_menu::main_menu, window_data::WindowData}};
use arc_swap::ArcSwap;
use imgui::{self, Context, FontId};
use raylib::{camera::Camera2D, math::Vector2, prelude::RaylibDrawHandle, RaylibHandle, RaylibThread};
use raylib_imgui_rs::{self, Renderer};
mod main_menu;
mod fonts;
mod window_data;
mod game;

#[allow(dead_code)]
pub struct Gui<'a> {
    state: GameState,
    channels: &'a GuiChannels,
    imgui: Context,
    fonts: HashMap<Font, FontId>,
    renderer: Renderer,
    imgui_cache: HashMap<ImguiCacheBuffer, Box<dyn ImguiCache>>,
    window_data: WindowData,
    camera: Camera2D
}

pub trait ImguiCache {}

impl <'a>Gui<'a> {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, channels: &'a GuiChannels) -> Self {
        let mut imgui = Context::create();
        let fonts = fonts(&mut imgui);
        let renderer = Renderer::create(&mut imgui,rl, &thread);
        let window_data = WindowData::new(rl);
        let camera = Camera2D {
            offset: Vector2::new(window_data.center_x(), window_data.center_y()),
            target: Vector2::new(0.0, 0.0),
            rotation: 0.0,
            zoom: 0.001
        };

        Gui { 
            state: GameState::MainMenu,
            channels: &channels,
            imgui,
            fonts,
            renderer,
            imgui_cache: HashMap::new(),
            window_data,
            camera
        }
    }

    pub fn tick(&mut self, rl: &mut RaylibDrawHandle, render_data: &ArcSwap<RenderData>) {
        self.window_data.update(rl);
        self.renderer.update(&mut self.imgui, rl);

        match self.channels.start_game_r.try_recv() {
            Ok(_) => self.state = GameState::Game,
            Err(_) => ()
        }

        match self.state {
            GameState::MainMenu => main_menu(self),
            GameState::Game => game(self, rl, render_data),
        }

        self.renderer.render(&mut self.imgui, rl);
    }
}