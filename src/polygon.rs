use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::line::line;
use std::cmp::{min, max};

/// Rellena un polígono usando el algoritmo de Scan-line.
pub fn fill_polygon(framebuffer: &mut Framebuffer, points: &[Vector2], color: Color) {
    if points.len() < 3 {
        return;
    }
    framebuffer.set_current_color(color);

    // 1. Encontrar los límites verticales (Y) del polígono.
    let mut y_min = points[0].y as i32;
    let mut y_max = points[0].y as i32;
    for point in points {
        y_min = min(y_min, point.y as i32);
        y_max = max(y_max, point.y as i32);
    }

    // 2. Iterar sobre cada línea de escaneo (scan-line) horizontal.
    for y in y_min..=y_max {
        let mut intersections = Vec::new();
        // 3. Encontrar todas las intersecciones de las aristas del polígono con la línea de escaneo actual.
        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];

            // Asegurarse de que la arista cruza la línea de escaneo
            if (p1.y <= y as f32 && p2.y > y as f32) || (p2.y <= y as f32 && p1.y > y as f32) {
                // Calcular la coordenada x de la intersección usando interpolación lineal.
                // Se evita la división por cero, aunque en este caso es poco probable.
                if (p2.y - p1.y).abs() > f32::EPSILON {
                    let x = p1.x + (y as f32 - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                    intersections.push(x as i32);
                }
            }
        }

        // 4. Ordenar las intersecciones por su coordenada x.
        intersections.sort();

        // 5. Rellenar los píxeles entre pares de intersecciones.
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];
                framebuffer.draw_horizontal_line(x_start, x_end, y);
            }
        }
    }
}

/// Dibuja solo el contorno de un polígono.
pub fn draw_polygon_outline(framebuffer: &mut Framebuffer, points: &[Vector2], color: Color) {
    if points.len() < 2 {
        return;
    }
    framebuffer.set_current_color(color);
    for i in 0..points.len() {
        let start = points[i];
        let end = points[(i + 1) % points.len()]; // Conecta con el siguiente y cierra el polígono al final
        line(framebuffer, start, end);
    }
}
