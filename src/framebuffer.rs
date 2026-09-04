use raylib::prelude::Color;

/// Framebuffer creado en la memoria RAM (CPU).
///
/// Cada pixel ocupa cuatro bytes en orden RGBA:
/// rojo, verde, azul y alfa. Raylib recibe posteriormente este arreglo
/// y lo copia a una textura para mostrarlo en la ventana.
pub struct Framebuffer {
    width: i32,
    height: i32,
    color_buffer: Vec<u8>,
}

impl Framebuffer {
    /// Crea un framebuffer del tamaño indicado y lo llena con un color inicial.
    pub fn new(width: i32, height: i32, background: Color) -> Self {
        assert!(
            width > 0 && height > 0,
            "El framebuffer debe tener un tamaño positivo"
        );

        let mut framebuffer = Self {
            width,
            height,
            // Cuatro componentes (RGBA) por cada pixel.
            color_buffer: vec![0; (width * height * 4) as usize],
        };

        framebuffer.clear(background);
        framebuffer
    }

    /// Pinta todo el framebuffer con un mismo color.
    pub fn clear(&mut self, color: Color) {
        // chunks_exact_mut(4) entrega un pixel RGBA en cada iteracion.
        for pixel in self.color_buffer.chunks_exact_mut(4) {
            pixel[0] = color.r;
            pixel[1] = color.g;
            pixel[2] = color.b;
            pixel[3] = color.a;
        }
    }

    /// Coloca un pixel. Si las coordenadas estan fuera de la pantalla,
    /// simplemente lo ignora para evitar accesos invalidos de memoria.
    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return;
        }

        // La posicion 2D se convierte en un indice lineal.
        // Se multiplica por cuatro porque cada pixel tiene cuatro bytes.
        let index = ((y * self.width + x) * 4) as usize;

        self.color_buffer[index] = color.r;
        self.color_buffer[index + 1] = color.g;
        self.color_buffer[index + 2] = color.b;
        self.color_buffer[index + 3] = color.a;
    }

    /// Expone los bytes para copiarlos a la textura de Raylib.
    /// La referencia es de solo lectura; nadie externo puede modificar el buffer.
    pub fn color_buffer(&self) -> &[u8] {
        &self.color_buffer
    }
}
