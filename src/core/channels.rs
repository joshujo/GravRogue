use std::process::Output;

use crossbeam::{self, channel::{unbounded, Receiver, Sender}};

use crate::core::channel_data::{Input, MainMenu};

pub struct Channel {
    pub main_menu_s: Sender<MainMenu>,
    pub main_menu_r: Receiver<MainMenu>,
    pub input_s: Sender<Input>,
    pub input_r: Receiver<Input>, 
    pub output_s: Sender<Output>,
    pub output_r: Receiver<Output>,
}

pub struct LogicChannels {
    pub main_menu_r: Receiver<MainMenu>,
    pub input_r: Receiver<Input>,
    pub output_s: Sender<Output>,
}

pub struct GuiChannels {
    pub main_menu_s: Sender<MainMenu>,
    pub input_s: Sender<Input>,
    pub output_r: Receiver<Output>,
}

impl Channel {
    pub fn new() -> Self {
        let (main_menu_s, main_menu_r) = unbounded::<MainMenu>();
        let (input_s, input_r) = unbounded::<Input>();
        let (output_s, output_r) = unbounded::<Output>();

        Channel {
            main_menu_s,
            main_menu_r,
            input_s,
            input_r,
            output_s,
            output_r
        }
    }

    pub fn logic_channels(&self) -> LogicChannels {
        LogicChannels {
            main_menu_r: self.main_menu_r.clone(),
            input_r: self.input_r.clone(),
            output_s: self.output_s.clone(),
        }
    }

    pub fn gui_channels(&self) -> GuiChannels {
        GuiChannels {
            main_menu_s: self.main_menu_s.clone(),
            input_s: self.input_s.clone(),
            output_r: self.output_r.clone(),
        }
    }
}