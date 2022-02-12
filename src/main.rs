extern crate sdl2;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use std::error::Error;
use std::time::Instant;

struct Point {
    x: f32,
    y: f32,
    alpha: f32,
}

struct Line {
    start: Point,
    end: Point,
}

struct Spiral {
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

    fn new(foreground: pixels::Color, angle_offset: f32, factor: f32) -> Self {
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

    fn compute_segments(&mut self, dt: f32) {
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

    fn render<T: DrawRenderer>(&mut self, canvas: &mut T) {
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

trait Lerp<Rhs = Self> {
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

fn get_point(theta: f32, factor: f32, angle_offset: f32, rate: f32) -> Point {
    let x = theta * factor * (theta + angle_offset).cos();
    let y = rate * theta;
    let z = -theta * factor * (theta + angle_offset).sin();

    let alpha = f32::min(
        1.0,
        ((y * factor / rate * 0.1 + 0.02 - z) * 40.0).atan() * 0.35 + 0.65,
    );
    project2d(x, y, z, alpha)
}

fn project2d(x: f32, y: f32, z: f32, a: f32) -> Point {
    const Y_SCREEN_OFFSET: f32 = 300.0;
    const X_SCREEN_OFFSET: f32 = 240.0;
    const X_SCREEN_SCALE: f32 = 700.0;
    const Y_SCREEN_SCALE: f32 = 700.0;
    const Y_CAMERA: f32 = 1.5;
    const Z_CAMERA: f32 = -5.0;
    Point {
        x: X_SCREEN_OFFSET + X_SCREEN_SCALE * (x / (z - Z_CAMERA)),
        y: Y_SCREEN_OFFSET + Y_SCREEN_SCALE * ((y - Y_CAMERA) / (z - Z_CAMERA)),
        alpha: a,
    }
}

fn d_theta(theta: f32, l_line_length: f32, rate: f32, factor: f32) -> f32 {
    l_line_length / (rate * rate + factor * factor * theta * theta).sqrt()
}

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_sys = sdl_context.video()?;
    let window = video_sys
        .window("rust-sdl2_gfx: draw line & FPSManager", 480, 800)
        .build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;
    let mut events = sdl_context.event_pump()?;
    let p_fmt: pixels::PixelFormat = pixels::PixelFormatEnum::RGBA8888.try_into().unwrap();
    let mut spirals = [
        (0x220000FF, 0.92, 0.9),
        (0x002211FF, 0.08, 0.9),
        (0x660000FF, 0.95, 0.93),
        (0x003322FF, 0.05, 0.93),
        (0xff0000FF, 1.0, 1.0),
        (0x00ffccFF, 0.0, 1.0),
    ]
    .map(|(c, a, f)| Spiral::new(pixels::Color::from_u32(&p_fmt, c), a, f));

    let mut timer = Instant::now();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        // dt calculation
        let dt = timer.elapsed().as_secs_f32();
        timer = Instant::now();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        for s in &mut spirals {
            s.compute_segments(dt);
            s.render(&mut canvas);
        }
        canvas.present();
    }

    Ok(())
}
