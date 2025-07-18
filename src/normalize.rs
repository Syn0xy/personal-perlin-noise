#[derive(Debug, Default, Clone, Copy)]
pub enum NormalizeMode {
    #[default]
    Global,
    Local,
}

pub fn normalize_map(
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

pub fn inverse_lerp(min: f32, max: f32, value: f32) -> f32 {
    (value - min) / (max - min)
}
