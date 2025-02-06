use noise::{NoiseFn, Perlin};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[derive(Debug)]
pub struct NoiseDescriptor {
    pub seed: u64,
    pub width: usize,
    pub height: usize,
    pub scale: f32,
    pub octaves: usize,
    pub persistance: f32,
    pub lacunarity: f32,
}

#[derive(Debug)]
pub enum NormalizeMode {
    Local,
    Global,
}

#[derive(Default)]
pub struct NoiseOffset {
    pub x: f32,
    pub y: f32,
}

pub fn generate_map(
    noise_descriptor: &NoiseDescriptor,
    offset: &NoiseOffset,
    normalize_mode: &NormalizeMode,
) -> Vec<f32> {
    let map_width = noise_descriptor.height;
    let map_height = noise_descriptor.height;
    let seed = noise_descriptor.seed;
    let mut scale = noise_descriptor.scale;
    let octaves = noise_descriptor.octaves;
    let persistance = noise_descriptor.persistance;
    let lacunarity = noise_descriptor.lacunarity;

    let map_length = map_width * map_height;
    let perlin = Perlin::new(seed as u32);
    let mut noise_map: Vec<f32> = Vec::new();
    let mut octave_offsets: Vec<NoiseOffset> = Vec::new();
    let mut rng = StdRng::seed_from_u64(seed);

    let mut max_possible_height: f32 = 0.0;
    let mut amplitude: f32 = 1.0;
    let mut frequency: f32;

    for _ in 0..octaves {
        let offset_x: f32 = rng.gen_range(-100_000_f32..100_000_f32) + offset.x;
        let offset_y: f32 = rng.gen_range(-100_000_f32..100_000_f32) - offset.y;
        octave_offsets.push(NoiseOffset {
            x: offset_x,
            y: offset_y,
        });

        max_possible_height += amplitude;
        amplitude *= persistance;
    }

    if scale <= 0.0 {
        scale = 0.00001_f32
    }

    let mut max_local_noise_height = f32::MIN;
    let mut min_local_noise_height = f32::MAX;

    let half_width = (map_width / 2) as f32;
    let half_height = (map_height / 2) as f32;

    for map_index in 0..map_length {
        let (x, y) = (
            (map_index / map_width) as f32,
            (map_index % map_width) as f32,
        );

        let mut noise_height = 0_f32;

        amplitude = 1.;
        frequency = 1.;

        for i in 0..octaves {
            let sample_x = (x - half_width + octave_offsets[i].x) / scale * frequency;
            let sample_y = (y - half_height + octave_offsets[i].y) / scale * frequency;
            let perlin_value = (perlin.get([sample_x as f64, sample_y as f64]) * 2. - 1.) as f32;

            noise_height += perlin_value * amplitude;
            amplitude *= persistance;
            frequency *= lacunarity;
        }

        if noise_height > max_local_noise_height {
            max_local_noise_height = noise_height;
        } else if noise_height < min_local_noise_height {
            min_local_noise_height = noise_height;
        }

        noise_map.push(noise_height);
    }

    normalize_map(
        &mut noise_map,
        map_length,
        max_possible_height,
        min_local_noise_height,
        max_local_noise_height,
        &normalize_mode,
    );

    noise_map
}

fn normalize_map(
    noise_map: &mut Vec<f32>,
    map_length: usize,
    max_possible_height: f32,
    min_local_noise_height: f32,
    max_local_noise_height: f32,
    normalize_mode: &NormalizeMode,
) {
    match normalize_mode {
        NormalizeMode::Local => {
            for map_index in 0..map_length {
                noise_map[map_index] = inverse_lerp(
                    min_local_noise_height,
                    max_local_noise_height,
                    noise_map[map_index],
                )
            }
        }
        NormalizeMode::Global => {
            for map_index in 0..map_length {
                noise_map[map_index] =
                    ((noise_map[map_index] + 1_f32) / max_possible_height).max(0_f32)
            }
        }
    }
}

fn inverse_lerp(min: f32, max: f32, value: f32) -> f32 {
    (value - min) / (max - min)
}
