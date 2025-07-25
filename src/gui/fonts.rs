use std::collections::HashMap;

use imgui::{Context, FontId, FontSource};

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Font {
    ChakraPetchBold,
    ChakraPetchTitle,
    ChakraPetchBoldItalic,
    ChakraPetchItalic,
    ChakraPetchLight,
    ChakraPetchLightItalic,
    ChakraPetchMedium,
    ChakraPetchMediumItalic,
    ChakraPetchRegular,
    ChakraPetchSemiBold,
    ChakraPetchSemiBoldItalic
}

pub fn fonts(imgui: &mut Context) -> HashMap<Font, FontId> {
    let mut hashmap = HashMap::new();

    let data = include_bytes!("../../fonts/Chakra_Petch/ChakraPetch-Regular.ttf");
    let chakra_petch_regular = imgui.fonts().add_font(&[FontSource::TtfData { data: data, size_pixels: 30.0, config: None }]);
    hashmap.insert(Font::ChakraPetchRegular, chakra_petch_regular);

    let data = include_bytes!("../../fonts/Chakra_Petch/ChakraPetch-Bold.ttf");
    let chakra_petch_bold = imgui.fonts().add_font(&[FontSource::TtfData { data: data, size_pixels: 30.0, config: None }]);
    hashmap.insert(Font::ChakraPetchBold, chakra_petch_bold);

    let data = include_bytes!("../../fonts/Chakra_Petch/ChakraPetch-Bold.ttf");
    let chakra_petch_title = imgui.fonts().add_font(&[FontSource::TtfData { data: data, size_pixels: 100.0, config: None }]);
    hashmap.insert(Font::ChakraPetchTitle, chakra_petch_title);

    let data = include_bytes!("../../fonts/Chakra_Petch/ChakraPetch-BoldItalic.ttf");
    let chakra_petch_bold_italic = imgui.fonts().add_font(&[FontSource::TtfData { data: data, size_pixels: 30.0, config: None }]);
    hashmap.insert(Font::ChakraPetchBoldItalic, chakra_petch_bold_italic);

    let data = include_bytes!("../../fonts/Chakra_Petch/ChakraPetch-Italic.ttf");
    let chakra_petch_italic = imgui.fonts().add_font(&[FontSource::TtfData { data: data, size_pixels: 30.0, config: None }]);
    hashmap.insert(Font::ChakraPetchItalic, chakra_petch_italic);

    let data = include_bytes!("../../fonts/Chakra_Petch/ChakraPetch-Light.ttf");
    let chakra_petch_light = imgui.fonts().add_font(&[FontSource::TtfData { data: data, size_pixels: 30.0, config: None }]);
    hashmap.insert(Font::ChakraPetchLight, chakra_petch_light);

    let data = include_bytes!("../../fonts/Chakra_Petch/ChakraPetch-LightItalic.ttf");
    let chakra_petch_light_italic = imgui.fonts().add_font(&[FontSource::TtfData { data: data, size_pixels: 30.0, config: None }]);
    hashmap.insert(Font::ChakraPetchLightItalic, chakra_petch_light_italic);

    let data = include_bytes!("../../fonts/Chakra_Petch/ChakraPetch-Medium.ttf");
    let chakra_petch_medium = imgui.fonts().add_font(&[FontSource::TtfData { data: data, size_pixels: 30.0, config: None }]);
    hashmap.insert(Font::ChakraPetchMedium, chakra_petch_medium);

    let data = include_bytes!("../../fonts/Chakra_Petch/ChakraPetch-MediumItalic.ttf");
    let chakra_petch_medium_italic = imgui.fonts().add_font(&[FontSource::TtfData { data: data, size_pixels: 30.0, config: None }]);
    hashmap.insert(Font::ChakraPetchMediumItalic, chakra_petch_medium_italic);

    let data = include_bytes!("../../fonts/Chakra_Petch/ChakraPetch-SemiBold.ttf");
    let chakra_petch_semi_bold = imgui.fonts().add_font(&[FontSource::TtfData { data: data, size_pixels: 30.0, config: None }]);
    hashmap.insert(Font::ChakraPetchSemiBold, chakra_petch_semi_bold);

    let data = include_bytes!("../../fonts/Chakra_Petch/ChakraPetch-SemiBoldItalic.ttf");
    let chakra_petch_semi_bold_italic = imgui.fonts().add_font(&[FontSource::TtfData { data: data, size_pixels: 30.0, config: None }]);
    hashmap.insert(Font::ChakraPetchSemiBoldItalic, chakra_petch_semi_bold_italic);

    hashmap
}