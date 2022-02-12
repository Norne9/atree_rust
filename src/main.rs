extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use std::error::Error;
use std::time::Instant;

mod line;
mod point2d;
mod spiral;
mod tools;

use spiral::Spiral;

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
