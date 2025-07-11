mod framebuffer;
mod line;
mod polygon;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use polygon::{fill_polygon, draw_polygon_outline};

fn main() {
    let (screen_width, screen_height) = (800, 600);

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Rasterizer - Poligono 1")
        .build();

    // El fondo es blanco, que también servirá como el color del borde.
    let mut framebuffer = Framebuffer::new(screen_width, screen_height, Color::WHITE);
    framebuffer.clear();

    // --- Polígono 1: Estrella (Amarillo con orilla blanca) ---
    let polygon1 = vec![
        Vector2::new(165.0, 380.0), Vector2::new(185.0, 360.0), Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0), Vector2::new(233.0, 330.0), Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0), Vector2::new(220.0, 385.0), Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];

    // 1. Rellenar el polígono de color amarillo.
    fill_polygon(&mut framebuffer, &polygon1, Color::YELLOW);

    // 2. Dibujar el contorno. Como el fondo ya es blanco, este paso es opcional
    // pero lo mantenemos por claridad si quisiéramos un borde de otro color.
    draw_polygon_outline(&mut framebuffer, &polygon1, Color::WHITE);

    // Guardar el resultado en un archivo para la entrega.
    framebuffer.render_to_file("out.bmp");

    // Cargar el framebuffer como una textura para mostrarlo en la ventana.
    let texture = rl.load_texture_from_image(&thread, &framebuffer.color_buffer).unwrap();

    // Bucle principal para mantener la ventana abierta.
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK); // Un fondo oscuro para la ventana, para que resalte el dibujo.
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
