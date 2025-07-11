use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        // Aseguramos que el formato sea compatible para manipulación de píxeles
        let mut color_buffer = Image::gen_image_color(width, height, background_color);
        color_buffer.set_format(PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        // Limpia el buffer con el color de fondo
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
        self.color_buffer.set_format(PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8);
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }
    
    /// Dibuja una línea horizontal desde x1 hasta x2 en la coordenada y.
    pub fn draw_horizontal_line(&mut self, x1: i32, x2: i32, y: i32) {
        for x in x1..=x2 {
            self.set_pixel(x, y);
        }
    }

    pub fn render_to_file(&self, file_path: &str) {
        // Exporta la imagen. La extensión del archivo determina el formato.
        self.color_buffer.export_image(file_path);
    }
}
