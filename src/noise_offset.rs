#[derive(Debug, Default)]
pub struct NoiseOffset {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for NoiseOffset {
    fn from(t: (f32, f32)) -> Self {
        Self { x: t.0, y: t.1 }
    }
}

impl From<[f32; 2]> for NoiseOffset {
    fn from(a: [f32; 2]) -> Self {
        Self { x: a[0], y: a[1] }
    }
}

impl From<NoiseOffset> for (f32, f32) {
    fn from(n: NoiseOffset) -> Self {
        (n.x, n.y)
    }
}

impl From<NoiseOffset> for [f32; 2] {
    fn from(n: NoiseOffset) -> Self {
        [n.x, n.y]
    }
}
