use crate::camera::ProjectPoint;
use crate::line::Line;
use crate::point::Point3d;
use crate::tools::Lerp;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;

pub struct Spiral {
    foreground: pixels::Color,
    angle_offset: f32,
    factor: f32,
    offset: f32,
    segment: Vec<Line>,
}

impl Spiral {
    const THETA_MIN: f32 = 0.0;
    const THETA_MAX: f32 = 8.0 * std::f32::consts::PI;
    const PERIOD: f32 = Self::THETA_MAX;
    const LINE_SPACING: f32 = 1.5 / 12.0;
    const LINE_LENGTH: f32 = Self::LINE_SPACING / 2.0;
    const G_RATE: f32 = 1.0 / (2.0 * std::f32::consts::PI);
    const G_FACTOR: f32 = Self::G_RATE / 3.0;
    const SPEED: f32 = Self::PERIOD * 1.5;

    pub fn new(foreground: pixels::Color, angle_offset: f32, factor: f32) -> Self {
        let angle_offset = angle_offset * std::f32::consts::PI;
        let factor = factor * Self::G_FACTOR;
        Self {
            foreground,
            angle_offset,
            factor,
            offset: 0.0,
            segment: vec![],
        }
    }

    pub fn compute_segment(&mut self, dt: f32) {
        self.offset += dt * Self::SPEED;
        if self.offset > Self::PERIOD {
            self.offset -= Self::PERIOD;
        }

        let mut theta = Self::THETA_MIN
            - d_theta(
                Self::THETA_MIN,
                Self::LINE_SPACING * self.offset / Self::PERIOD,
                Self::G_RATE,
                self.factor,
            );

        self.segment.clear();

        while theta < Self::THETA_MAX {
            let theta_old = f32::max(theta, Self::THETA_MIN);
            theta += d_theta(theta, Self::LINE_LENGTH, Self::G_RATE, self.factor);
            let theta_end = (theta_old + f32::max(theta, Self::THETA_MIN)) / 2.0;
            if theta_end < Self::THETA_MIN {
                continue;
            }

            let start = get_point(theta_old, self.factor, self.angle_offset, Self::G_RATE);
            let end = get_point(theta_end, self.factor, self.angle_offset, Self::G_RATE);
            let alpha = get_alpha(&start, self.factor, Self::G_RATE);
            self.segment.push(Line { start, end, alpha });
        }
    }

    pub fn render<T: DrawRenderer, U: ProjectPoint>(
        &self,
        canvas: &mut T,
        camera: &U,
    ) -> Result<(), String> {
        for line in &self.segment {
            let color = pixels::Color::BLACK.lerp(&self.foreground, line.alpha);
            line.draw(canvas, camera, color)?;
        }
        Ok(())
    }
}

fn get_alpha(point: &Point3d, factor: f32, rate: f32) -> f32 {
    f32::min(
        1.0,
        ((point.y * factor / rate * 0.1 + 0.02 - point.z) * 40.0).atan() * 0.35 + 0.65,
    )
}

fn get_point(theta: f32, factor: f32, angle_offset: f32, rate: f32) -> Point3d {
    let x = theta * factor * (theta + angle_offset).cos();
    let y = rate * theta;
    let z = -theta * factor * (theta + angle_offset).sin();
    Point3d { x, y, z }
}

fn d_theta(theta: f32, l_line_length: f32, rate: f32, factor: f32) -> f32 {
    l_line_length / (rate * rate + factor * factor * theta * theta).sqrt()
}
