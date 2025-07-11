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
        .title("Rasterizer - Poligono 2")
        .build();

    // El fondo es blanco, que también servirá como el color del borde.
    let mut framebuffer = Framebuffer::new(screen_width, screen_height, Color::WHITE);
    framebuffer.clear();

    // --- Polígono 2: Rombo (Azul con orilla blanca) ---
    let polygon2 = vec![
        Vector2::new(321.0, 335.0), Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0), Vector2::new(374.0, 302.0),
    ];

    // 1. Rellenar el polígono de color azul.
    fill_polygon(&mut framebuffer, &polygon2, Color::BLUE);

    // 2. Dibujar el contorno blanco (opcional, ya que el fondo es blanco).
    draw_polygon_outline(&mut framebuffer, &polygon2, Color::WHITE);

    // Guardar el resultado en un archivo para la entrega.
    framebuffer.render_to_file("out.bmp");

    // Cargar el framebuffer como una textura para mostrarlo en la ventana.
    let texture = rl.load_texture_from_image(&thread, &framebuffer.color_buffer).unwrap();

    // Bucle principal para mantener la ventana abierta.
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
