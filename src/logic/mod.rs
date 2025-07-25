use crate::core::{channel_data::StartGame, channels::LogicChannels};

pub fn logic(logic_channels: &LogicChannels) {
    loop {
        let _ = logic_channels.main_menu_r.recv().unwrap();
        println!("Logic thread received start game");
        logic_channels.start_game_s.send(StartGame::StartGame).unwrap();
        loop {
            
        }
    }
}