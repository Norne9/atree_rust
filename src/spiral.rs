use crate::line::Line;
use crate::point2d::Point2d;
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
    const PERIOD: f32 = 20.0;
    const LINE_SPACING: f32 = 1.5 / 12.0;
    const LINE_LENGTH: f32 = Self::LINE_SPACING / 2.0;
    const G_RATE: f32 = 1.0 / (2.0 * std::f32::consts::PI);
    const G_FACTOR: f32 = Self::G_RATE / 3.0;
    const SPEED: f32 = 12.0;

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

    pub fn compute_segments(&mut self, dt: f32) {
        self.offset += dt * Self::SPEED;
        if self.offset > Self::PERIOD {
            self.offset -= Self::PERIOD;
        }

        let mut theta = Self::THETA_MIN
            + d_theta(
                Self::THETA_MIN,
                Self::LINE_SPACING * self.offset / Self::PERIOD,
                Self::G_RATE,
                self.factor,
            );

        self.segment.clear();
        self.segment.push(Line {
            start: get_point(0.0, self.factor, self.angle_offset, Self::G_RATE),
            end: get_point(theta / 2.0, self.factor, self.angle_offset, Self::G_RATE),
        });
        while theta < Self::THETA_MAX {
            let theta_old = theta;
            theta += d_theta(theta, Self::LINE_LENGTH, Self::G_RATE, self.factor);

            self.segment.push(Line {
                start: get_point(theta_old, self.factor, self.angle_offset, Self::G_RATE),
                end: get_point(
                    (theta_old + theta) / 2.0,
                    self.factor,
                    self.angle_offset,
                    Self::G_RATE,
                ),
            });
        }
    }

    pub fn render<T: DrawRenderer>(&mut self, canvas: &mut T) {
        for line in &self.segment {
            let color = pixels::Color::BLACK.lerp(&self.foreground, line.start.alpha);
            canvas.line(
                line.start.x as i16,
                line.start.y as i16,
                line.end.x as i16,
                line.end.y as i16,
                color,
            );
        }
    }
}

fn get_point(theta: f32, factor: f32, angle_offset: f32, rate: f32) -> Point2d {
    let x = theta * factor * (theta + angle_offset).cos();
    let y = rate * theta;
    let z = -theta * factor * (theta + angle_offset).sin();

    let alpha = f32::min(
        1.0,
        ((y * factor / rate * 0.1 + 0.02 - z) * 40.0).atan() * 0.35 + 0.65,
    );
    project2d(x, y, z, alpha)
}

fn project2d(x: f32, y: f32, z: f32, a: f32) -> Point2d {
    const Y_SCREEN_OFFSET: f32 = 300.0;
    const X_SCREEN_OFFSET: f32 = 240.0;
    const X_SCREEN_SCALE: f32 = 700.0;
    const Y_SCREEN_SCALE: f32 = 700.0;
    const Y_CAMERA: f32 = 1.5;
    const Z_CAMERA: f32 = -5.0;
    Point2d {
        x: X_SCREEN_OFFSET + X_SCREEN_SCALE * (x / (z - Z_CAMERA)),
        y: Y_SCREEN_OFFSET + Y_SCREEN_SCALE * ((y - Y_CAMERA) / (z - Z_CAMERA)),
        alpha: a,
    }
}

fn d_theta(theta: f32, l_line_length: f32, rate: f32, factor: f32) -> f32 {
    l_line_length / (rate * rate + factor * factor * theta * theta).sqrt()
}
