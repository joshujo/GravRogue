use crate::core::channels::LogicChannels;

pub fn logic(logic_channels: &LogicChannels) {
    loop {
        let _ = logic_channels.main_menu_r.recv().unwrap();

        loop {
            
        }
    }
}