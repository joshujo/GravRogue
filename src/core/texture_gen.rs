use std::{ f64::consts::PI, io::Cursor };

use image::{
    codecs::{bmp::BmpEncoder, png::PngEncoder},
    DynamicImage,
    ExtendedColorType,
    GenericImageView,
    ImageBuffer,
    ImageEncoder,
    ImageReader,
    Rgba,
};
use noise::{ NoiseFn, Perlin };

pub fn generate_planet_texture() -> Vec<u8> {
    let size = 256;
    let perlin = Perlin::new(rand::random());
    let mut buffer = Vec::with_capacity(((size as i32) * (size as i32) * 4) as usize);
    let scale = 1.0;

    for y in 0..size as i32 {
        for x in 0..size as i32 {
            let nx = ((x as f64) / (size as f64)) * 2.0 - 1.0;
            let ny = ((y as f64) / (size as f64)) * 2.0 - 1.0;
            let dist = (nx * nx + ny * ny).sqrt();

            if dist > 1.0 {
                buffer.extend_from_slice(&[0, 0, 0, 0]);
                continue;
            }

            let theta = nx * PI;
            let phi = (ny * PI) / 2.0;

            let val = perlin.get([theta.sin() * scale, phi.cos() * scale, 0.0]);

            let color = if val > 0.4 {
                [150, 150, 150]
            } else if val > 0.2 {
                [34, 139, 34]
            } else if val > 0.0 {
                [222, 184, 135]
            } else {
                [30, 60, 120]
            };

            buffer.extend_from_slice(&[color[0], color[1], color[2], 255]);
        }
    }

    let mut png_data = vec![];

    {
        let encoder = BmpEncoder::new(&mut png_data);

        BmpEncoder::write_image(encoder, &buffer, size, size, ExtendedColorType::Rgba8).unwrap();
    }
    png_data
}
