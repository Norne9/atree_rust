extern crate sdl2;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use std::error::Error;
use std::time::Instant;

mod camera;
mod line;
mod point;
mod spiral;
mod tools;

use camera::Camera;
use spiral::Spiral;

fn main() -> Result<(), Box<dyn Error>> {
    const WIDTH: u32 = 480;
    const HEIGHT: u32 = 800;

    let sdl_context = sdl2::init()?;
    let video_sys = sdl_context.video()?;
    let window = video_sys
        .window("Christmas Tree", WIDTH, HEIGHT)
        .resizable()
        .build()?;
    let mut canvas = window.into_canvas().present_vsync().build()?;
    let mut events = sdl_context.event_pump()?;
    let p_fmt = pixels::PixelFormatEnum::RGBA8888.try_into()?;
    let mut spirals = [
        (0x220000ff, 0.92, 0.9),
        (0x002211ff, 0.08, 0.9),
        (0x660000ff, 0.95, 0.93),
        (0x003322ff, 0.05, 0.93),
        (0xff0000ff, 1.0, 1.0),
        (0x00ffccff, 0.0, 1.0),
    ]
    .map(|(c, a, f)| Spiral::new(pixels::Color::from_u32(&p_fmt, c), a, f));

    let mut camera = Camera::new(WIDTH, HEIGHT, 1.0, 2.1, -5.0);

    let mut timer = Instant::now();

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => camera.resize(w, h),
                _ => {}
            }
        }

        // dt calculation
        let dt = timer.elapsed().as_secs_f32();
        timer = Instant::now();

        canvas.set_draw_color(pixels::Color::BLACK);
        canvas.clear();
        for s in &mut spirals {
            s.compute_segment(dt);
            s.render(&mut canvas, &camera)?;
        }
        canvas.present();
    }

    Ok(())
}
