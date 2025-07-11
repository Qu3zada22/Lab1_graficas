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
        .title("Rasterizer - Poligono 4 con Agujero") // Título actualizado
        .build();

    // El fondo es blanco, que también servirá como el color del borde.
    let mut framebuffer = Framebuffer::new(screen_width, screen_height, Color::WHITE);

    framebuffer.clear();

    // --- Polígono 4: Grande (Verde con orilla blanca) ---
    // Coordenadas del Polígono 4:
    // (413, 177) (448, 159) (502, 88) (553, 53) (535, 36) (676, 37) (660, 52)
    // (750, 145) (761, 179) (672, 192) (659, 214) (615, 214) (632, 230) (580, 230)
    // (597, 215) (552, 214) (517, 144) (466, 180)
    let polygon4 = vec![
        Vector2::new(413.0, 177.0), Vector2::new(448.0, 159.0),
        Vector2::new(502.0, 88.0),  Vector2::new(553.0, 53.0),
        Vector2::new(535.0, 36.0),  Vector2::new(676.0, 37.0),
        Vector2::new(660.0, 52.0),  Vector2::new(750.0, 145.0),
        Vector2::new(761.0, 179.0), Vector2::new(672.0, 192.0),
        Vector2::new(659.0, 214.0), Vector2::new(615.0, 214.0),
        Vector2::new(632.0, 230.0), Vector2::new(580.0, 230.0),
        Vector2::new(597.0, 215.0), Vector2::new(552.0, 214.0),
        Vector2::new(517.0, 144.0), Vector2::new(466.0, 180.0),
    ];

    // --- Polígono 5: Agujero (Rellenado con color de fondo) ---
    // Coordenadas del Polígono 5: (682, 175) (708, 120) (735, 148) (739, 170)
    let polygon5 = vec![
        Vector2::new(682.0, 175.0),
        Vector2::new(708.0, 120.0),
        Vector2::new(735.0, 148.0),
        Vector2::new(739.0, 170.0),
    ];

    // 1. Rellenar el Polígono 4 de color verde.
    fill_polygon(&mut framebuffer, &polygon4, Color::GREEN);

    // 2. Rellenar el Polígono 5 (el agujero) con el color de fondo (blanco).
    fill_polygon(&mut framebuffer, &polygon5, Color::WHITE);

    // 3. Dibujar el contorno blanco del Polígono 4.
    draw_polygon_outline(&mut framebuffer, &polygon4, Color::WHITE);

    // 4. Dibujar el contorno blanco del Polígono 5.
    draw_polygon_outline(&mut framebuffer, &polygon5, Color::WHITE);

    // Guardar el resultado en un archivo para la entrega.
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
