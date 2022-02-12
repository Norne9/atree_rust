use sdl2::pixels;

pub trait Lerp<Rhs = Self> {
    fn lerp(&self, other: &Rhs, alpha: f32) -> Self;
}

impl Lerp for pixels::Color {
    fn lerp(&self, col: &Self, alpha: f32) -> Self {
        let c1 = [self.r as f32, self.g as f32, self.b as f32, self.a as f32];
        let c2 = [col.r as f32, col.g as f32, col.b as f32, col.a as f32];
        let mut r = c1
            .into_iter()
            .zip(c2)
            .map(|(a, b)| (a * (1.0 - alpha) + b * alpha) as u8);
        pixels::Color::RGBA(
            r.next().unwrap(),
            r.next().unwrap(),
            r.next().unwrap(),
            r.next().unwrap(),
        )
    }
}
