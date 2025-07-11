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
        .title("Rasterizer - Poligono 3") // Título actualizado para Polígono 3
        .build();

    // El fondo es blanco, que también servirá como el color del borde.
    let mut framebuffer = Framebuffer::new(screen_width, screen_height, Color::WHITE);

    framebuffer.clear();

    // --- Polígono 3: Triángulo (Rojo con orilla blanca) ---
    // Coordenadas del Polígono 3: (377, 249) (411, 197) (436, 249)
    let polygon3 = vec![
        Vector2::new(377.0, 249.0),
        Vector2::new(411.0, 197.0),
        Vector2::new(436.0, 249.0),
    ];

    // 1. Rellenar el polígono de color rojo.
    fill_polygon(&mut framebuffer, &polygon3, Color::RED);

    // 2. Dibujar el contorno blanco.
    draw_polygon_outline(&mut framebuffer, &polygon3, Color::WHITE);

    // Guardar el resultado en un archivo para la entrega.
    // Asegúrate de que el nombre del archivo sea 'out.bmp' como se especifica en las instrucciones.
    framebuffer.render_to_file("out.bmp");

    // Cargar el framebuffer como una textura para mostrarlo en la ventana.
    let texture = rl.load_texture_from_image(&thread, &framebuffer.color_buffer).unwrap();

    // Bucle principal para mantener la ventana abierta.
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK); // Fondo de la ventana de Raylib
        d.draw_texture(&texture, 0, 0, Color::WHITE); // Dibuja el contenido del framebuffer
    }
}
