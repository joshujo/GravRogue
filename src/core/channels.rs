use std::process::Output;

use crossbeam::{self, channel::{unbounded, Receiver, Sender}};

use crate::core::channel_data::{Input, MainMenu, StartGame};

pub struct Channel {
    pub main_menu_s: Sender<MainMenu>,
    pub main_menu_r: Receiver<MainMenu>,
    pub start_game_s: Sender<StartGame>,
    pub start_game_r: Receiver<StartGame>,
    pub input_s: Sender<Input>,
    pub input_r: Receiver<Input>, 
    pub output_s: Sender<Output>,
    pub output_r: Receiver<Output>,
}

pub struct LogicChannels {
    pub main_menu_r: Receiver<MainMenu>,
    pub start_game_s: Sender<StartGame>,
    pub input_r: Receiver<Input>,
    pub output_s: Sender<Output>,
}

pub struct GuiChannels {
    pub main_menu_s: Sender<MainMenu>,
    pub start_game_r: Receiver<StartGame>,
    pub input_s: Sender<Input>,
    pub output_r: Receiver<Output>,
}

impl Channel {
    pub fn new() -> Self {
        let (main_menu_s, main_menu_r) = unbounded::<MainMenu>();
        let (start_game_s, start_game_r) = unbounded::<StartGame>();
        let (input_s, input_r) = unbounded::<Input>();
        let (output_s, output_r) = unbounded::<Output>();

        Channel {
            main_menu_s,
            main_menu_r,
            start_game_s,
            start_game_r,
            input_s,
            input_r,
            output_s,
            output_r
        }
    }

    pub fn logic_channels(&self) -> LogicChannels {
        LogicChannels {
            main_menu_r: self.main_menu_r.clone(),
            start_game_s: self.start_game_s.clone(),
            input_r: self.input_r.clone(),
            output_s: self.output_s.clone(),
        }
    }

    pub fn gui_channels(&self) -> GuiChannels {
        GuiChannels {
            main_menu_s: self.main_menu_s.clone(),
            start_game_r: self.start_game_r.clone(),
            input_s: self.input_s.clone(),
            output_r: self.output_r.clone(),
        }
    }
}