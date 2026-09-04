use raylib::prelude::Color;

use crate::framebuffer::Framebuffer;

/// Dibuja un circulo relleno, pixel por pixel, sobre el framebuffer.
/// Este circulo sera la base que mas adelante convertiremos en una esfera 3D.
pub fn draw_circle(
    framebuffer: &mut Framebuffer,
    center_x: i32,
    center_y: i32,
    radius: i32,
    color: Color,
) {
    if radius <= 0 {
        return;
    }

    let radius_squared = radius * radius;

    // Recorremos el cuadrado que contiene al circulo.
    for offset_y in -radius..=radius {
        for offset_x in -radius..=radius {
            // Ecuacion del circulo: x^2 + y^2 <= radio^2.
            let distance_squared = offset_x * offset_x + offset_y * offset_y;

            if distance_squared <= radius_squared {
                framebuffer.put_pixel(center_x + offset_x, center_y + offset_y, color);
            }
        }
    }
}
