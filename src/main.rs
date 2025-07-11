mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::line;
use polygon::draw_polygon;

fn main() {
    let (screen_width, screen_height) = (800, 600);

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Rasterizer - Polygons")
        .build();

    let mut framebuffer = Framebuffer::new(screen_width, screen_height, Color::WHITE);
    framebuffer.clear();

    // Dibujo de líneas de prueba
    framebuffer.set_current_color(Color::BLACK);
    line(&mut framebuffer, Vector2::new(50.0, 50.0), Vector2::new(300.0, 100.0));
    line(&mut framebuffer, Vector2::new(50.0, 150.0), Vector2::new(300.0, 300.0));
    line(&mut framebuffer, Vector2::new(300.0, 150.0), Vector2::new(50.0, 300.0));
    line(&mut framebuffer, Vector2::new(400.0, 400.0), Vector2::new(600.0, 400.0));

    // Polígonos
    let polygon1 = vec![
        Vector2::new(100.0, 100.0),
        Vector2::new(200.0, 80.0),
        Vector2::new(250.0, 150.0),
        Vector2::new(200.0, 250.0),
        Vector2::new(100.0, 200.0),
    ];

    let polygon2 = vec![
        Vector2::new(400.0, 100.0),
        Vector2::new(450.0, 50.0),
        Vector2::new(500.0, 150.0),
        Vector2::new(470.0, 250.0),
        Vector2::new(380.0, 200.0),
    ];

    framebuffer.set_current_color(Color::RED);
    draw_polygon(&mut framebuffer, &polygon1);
    draw_polygon(&mut framebuffer, &polygon2);

    for point in polygon1.iter().chain(polygon2.iter()) {
        framebuffer.color_buffer.draw_circle(point.x as i32, point.y as i32, 3, Color::RED);
    }

    // Aquí cargamos la textura dentro de un bloque para controlar vida útil
    let texture = unsafe { rl.load_texture_from_image(&thread, &framebuffer.color_buffer).unwrap() };

    // Dibujar y mantener ventana abierta hasta cerrar
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }

    // Textura se liberará automáticamente cuando salga de scope
}
