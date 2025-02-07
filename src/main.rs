use std::u8;

use image::GrayImage;
use perlin_noise::{NoiseDescriptor, NoiseOffset, NormalizeMode};

fn main() {
    let noise_desc = NoiseDescriptor {
        seed: 0,
        width: 256,
        height: 256,
        scale: 50.0,
        octaves: 2,
        persistance: 0.5,
        lacunarity: 2.0,
    };

    generate_with_output(&noise_desc, &NormalizeMode::Global);
    generate_with_output(&noise_desc, &NormalizeMode::Local);
}

fn generate_with_output(noise_descriptor: &NoiseDescriptor, normalize_mode: &NormalizeMode) {
    let noise_map =
        perlin_noise::generate_map(noise_descriptor, &NoiseOffset::default(), normalize_mode);

    let to_image = noise_map
        .into_iter()
        .map(|v| (v * u8::MAX as f32) as u8)
        .collect();

    save_image(
        to_image,
        noise_descriptor.width as u32,
        noise_descriptor.height as u32,
        &format!("output_{normalize_mode:?}.png").to_lowercase(),
    );
}

fn save_image(data: Vec<u8>, width: u32, height: u32, filename: &str) {
    GrayImage::from_vec(width, height, data)
        .expect("Invalid dimensions for image creation")
        .save(filename)
        .expect("Failed to save image");
}
