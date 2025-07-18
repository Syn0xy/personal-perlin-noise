#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct NoiseOffset {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for NoiseOffset {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl From<[f32; 2]> for NoiseOffset {
    fn from([x, y]: [f32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<NoiseOffset> for (f32, f32) {
    fn from(NoiseOffset { x, y }: NoiseOffset) -> Self {
        (x, y)
    }
}

impl From<NoiseOffset> for [f32; 2] {
    fn from(NoiseOffset { x, y }: NoiseOffset) -> Self {
        [x, y]
    }
}
