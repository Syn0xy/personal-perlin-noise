use std::u8;

use image::GrayImage;
use perlin_noise::{NoiseDescriptor, NoiseOffset, NormalizeMode};

fn main() {
    let noise_desc_global = NoiseDescriptor {
        seed: 0,
        scale: 50.0,
        octaves: 2,
        persistance: 0.5,
        lacunarity: 2.0,
        normalize_mode: NormalizeMode::Global,
    };

    let noise_desc_local = NoiseDescriptor {
        normalize_mode: NormalizeMode::Local,
        ..noise_desc_global
    };

    generate_with_output(noise_desc_global);
    generate_with_output(noise_desc_local);
}

fn generate_with_output(noise_descriptor: NoiseDescriptor) {
    let map_width = 256;
    let map_height = 256;
    let noise_map = perlin_noise::generate_map(
        map_width,
        map_height,
        noise_descriptor,
        &NoiseOffset::default(),
    );

    let to_image = noise_map
        .into_iter()
        .map(|v| (v * u8::MAX as f32) as u8)
        .collect();
    let image_name = format!("output_{:?}.png", noise_descriptor.normalize_mode).to_lowercase();

    save_image(to_image, map_width, map_height, image_name);
}

fn save_image(data: Vec<u8>, width: u32, height: u32, filename: String) {
    GrayImage::from_vec(width, height, data)
        .expect("Invalid dimensions for image creation")
        .save(filename)
        .expect("Failed to save image");
}
