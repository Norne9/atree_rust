use crate::camera::ProjectPoint;
use crate::point::Point3d;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;

pub struct Line {
    pub start: Point3d,
    pub end: Point3d,
    pub alpha: f32,
}

impl Line {
    pub fn draw<T: DrawRenderer, U: ProjectPoint>(
        &self,
        canvas: &mut T,
        camera: &U,
        color: Color,
    ) -> Result<(), String> {
        let start = camera.project(&self.start);
        let end = camera.project(&self.end);
        canvas.aa_line(
            start.x.round() as i16,
            start.y.round() as i16,
            end.x.round() as i16,
            end.y.round() as i16,
            color,
        )
    }
}
