use crate::point::{Point2d, Point3d};

pub struct Camera {
    y_screen_offset: f32,
    x_screen_offset: f32,
    screen_scale: f32,
    scale: f32,
    y_camera: f32,
    z_camera: f32,
    width: f32,
    height: f32
}

pub trait ProjectPoint {
    fn project(&self, point: &Point3d) -> Point2d;
}

impl Camera {
    pub fn new(width: f32, height: f32, scale: f32, y_pos: f32, z_pos: f32) -> Self {
        Self {
            y_screen_offset: height / 2.0,
            x_screen_offset: width / 2.0,
            screen_scale: width.max(height)  / scale,
            scale,
            y_camera: y_pos,
            z_camera: z_pos,
            width, height
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        if (self.width - width).abs() + (self.height - height).abs() < 1.0 {
            return;
        }
        self.y_screen_offset = height / 2.0;
        self.x_screen_offset = width / 2.0;
        self.screen_scale = width.max(height) / self.scale;
    }
}

impl ProjectPoint for Camera {
    fn project(&self, point: &Point3d) -> Point2d {
        Point2d {
            x: self.x_screen_offset + self.screen_scale * (point.x / (point.z - self.z_camera)),
            y: self.y_screen_offset
                + self.screen_scale * ((point.y - self.y_camera) / (point.z - self.z_camera)),
        }
    }
}
