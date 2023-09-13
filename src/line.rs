use crate::camera::ProjectPoint;
use crate::point::Point3d;
use macroquad::prelude::*;

pub struct Line {
    pub start: Point3d,
    pub end: Point3d,
    pub alpha: f32,
}

impl Line {
    pub fn draw<U: ProjectPoint>(
        &self,
        camera: &U,
        color: Color,
    ) {
        let start = camera.project(&self.start);
        let end = camera.project(&self.end);
        let color = Color::new(color.r, color.g, color.b, self.alpha);
        draw_line(start.x, start.y, end.x, end.y, 2.0, color);
    }
}
