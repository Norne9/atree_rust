use macroquad::prelude::*;

mod camera;
mod line;
mod point;
mod spiral;

use camera::Camera;
use spiral::Spiral;

#[macroquad::main("BasicShapes")]
async fn main() {
    const WIDTH: f32 = 480.0;
    const HEIGHT: f32 = 800.0;

    request_new_screen_size(WIDTH, HEIGHT);

    let mut spirals = [
        (0x220000, 0.92, 0.9),
        (0x002211, 0.08, 0.9),
        (0x660000, 0.95, 0.93),
        (0x003322, 0.05, 0.93),
        (0xff0000, 1.0, 1.0),
        (0x00ffcc, 0.0, 1.0),
    ]
        .map(|(c, a, f)| Spiral::new(Color::from_hex(c), a, f));

    let mut camera = Camera::new(WIDTH, HEIGHT, 1.0, 2.1, -5.0);

    loop {
        clear_background(BLACK);
        camera.resize(screen_width(), screen_height());

        for s in &mut spirals {
            s.compute_segment(get_frame_time());
            s.render(&camera);
        }

        next_frame().await
    }
}