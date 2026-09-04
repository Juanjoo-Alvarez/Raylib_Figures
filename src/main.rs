use raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO;
use raylib::prelude::*;

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 900;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Esfera 3D - Juan Jose Rivas Alvarez - 24856")
        .build();

    rl.set_target_fps(60);

    // La camara existe en el mundo 3D, por eso usa vectores (x, y, z).
    let camera = Camera3D::perspective(
        Vector3::new(4.5, 3.2, 6.0), // Posicion de la camara
        Vector3::new(0.0, 1.0, 0.0), // Punto hacia el que observa
        Vector3::new(0.0, 1.0, 0.0), // Direccion hacia arriba
        45.0,                        // Campo de vision
    );

    // La luz permanece fija arriba y a un lado para que el sombreado se note.
    let light_position = Vector3::new(-2.0, 5.0, 2.0);

    // El shader calcula la iluminacion de la textura usando las normales.
    // Se declara antes del modelo para que viva mientras el modelo lo utiliza.
    let mut lighting_shader = rl.load_shader(
        &thread,
        Some("resources/lighting.vs"),
        Some("resources/lighting.fs"),
    );

    let light_position_location = lighting_shader.get_shader_location("lightPosition");
    let light_color_location = lighting_shader.get_shader_location("lightColor");
    let ambient_color_location = lighting_shader.get_shader_location("ambientColor");

    lighting_shader.set_shader_value(light_position_location, light_position);
    lighting_shader.set_shader_value(light_color_location, Vector3::new(1.0, 1.0, 1.0));
    lighting_shader.set_shader_value(ambient_color_location, Vector3::new(0.08, 0.08, 0.10));

    let mesh = Mesh::gen_mesh_sphere(&thread, 1.0, 32, 32);

    // El modelo pasa a ser el propietario de la malla.
    let mesh = unsafe { mesh.make_weak() };

    let mut sphere_model = rl
        .load_model_from_mesh(&thread, mesh)
        .expect("No se pudo cargar el modelo de la esfera");

    // Un color naranja plano permite ver claramente la zona iluminada
    // y la zona en sombra, sin detalles de una fotografia que distraigan.
    sphere_model.materials_mut()[0]
        .set_map_color(MATERIAL_MAP_ALBEDO, Color::new(235, 95, 35, 255));

    // El material combina la textura con nuestro shader de iluminacion.
    sphere_model.materials_mut()[0].set_shader(&lighting_shader);

    // La esfera esta una unidad arriba del suelo.
    let sphere_position = Vector3::new(0.0, 1.0, 0.0);

    while !rl.window_should_close() {
        let mut drawing = rl.begin_drawing(&thread);
        drawing.clear_background(Color::new(8, 12, 22, 255));

        {
            // Las figuras de este bloque se proyectan usando la camara 3D.
            let mut mode_3d = drawing.begin_mode3D(camera);

            mode_3d.draw_model(&mut sphere_model, sphere_position, 1.0, Color::WHITE);

            // Esta esfera pequena muestra la posicion de la luz.
            mode_3d.draw_sphere(light_position, 0.15, Color::YELLOW);

            // El suelo y la cuadricula dan una referencia clara de profundidad.
            mode_3d.draw_plane(
                Vector3::new(0.0, -0.02, 0.0),
                Vector2::new(12.0, 12.0),
                Color::new(28, 34, 48, 255),
            );
            mode_3d.draw_grid(12, 1.0);
        }

        drawing.draw_text("MATERIAL CON ILUMINACION", 20, 20, 22, Color::RAYWHITE);
        drawing.draw_text(
            "La esfera amarilla indica la posicion de la luz",
            20,
            52,
            18,
            Color::LIGHTGRAY,
        );
        drawing.draw_text("ESC: salir", 20, HEIGHT - 35, 18, Color::LIGHTGRAY);
        drawing.draw_fps(WIDTH - 95, 10);
    }
}
