use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;

pub fn draw_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    if points.len() < 2 {
        return;
    }

    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()]; // conecta con el siguiente y cierra al final
        line(framebuffer, start, end);
    }
}
