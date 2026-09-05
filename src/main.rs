use raylib::consts::MaterialMapIndex::MATERIAL_MAP_ALBEDO;
use raylib::prelude::*;

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 900;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Cubo con luz difusa - Juan Jose Rivas Alvarez - 24856")
        .build();

    rl.set_target_fps(60);

    // La camara existe en el mundo 3D, por eso usa vectores (x, y, z).
    let camera_position = Vector3::new(4.5, 3.2, 6.0);
    let camera = Camera3D::perspective(
        camera_position,             // Posicion de la camara
        Vector3::new(0.0, 1.0, 0.0), // Punto hacia el que observa
        Vector3::new(0.0, 1.0, 0.0), // Direccion hacia arriba
        45.0,                        // Campo de vision
    );

    // La luz gira alrededor del cubo, pero siempre permanece arriba.
    let mut light_position = Vector3::new(3.0, 5.0, 0.0);
    let mut light_angle = 0.0_f32;
    let mut rotation_angle = 0.0_f32;
    let mut animate_light = true;

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
    let view_position_location = lighting_shader.get_shader_location("viewPosition");
    let shininess_location = lighting_shader.get_shader_location("shininess");
    let specular_strength_location = lighting_shader.get_shader_location("specularStrength");

    lighting_shader.set_shader_value(light_position_location, light_position);
    lighting_shader.set_shader_value(light_color_location, Vector3::new(1.15, 1.05, 0.95));
    lighting_shader.set_shader_value(ambient_color_location, Vector3::new(0.04, 0.05, 0.08));
    lighting_shader.set_shader_value(view_position_location, camera_position);

    // El cubo comienza con una apariencia lisa y brillante.
    let mut shininess = 24.0_f32;
    let mut specular_strength = 2.2_f32;
    let mut material_name = "LISO / PULIDO";

    // Creamos un cubo de dos unidades por lado.
    let mesh = Mesh::gen_mesh_cube(&thread, 2.0, 2.0, 2.0);

    // El modelo pasa a ser el propietario de la malla.
    let mesh = unsafe { mesh.make_weak() };

    let mut cube_model = rl
        .load_model_from_mesh(&thread, mesh)
        .expect("No se pudo cargar el modelo del cubo");

    // El color plano permite distinguir la intensidad difusa de cada cara.
    cube_model.materials_mut()[0].set_map_color(MATERIAL_MAP_ALBEDO, Color::new(25, 80, 190, 255));

    cube_model.materials_mut()[0].set_shader(&lighting_shader);

    // El centro queda una unidad arriba para apoyar el cubo sobre el suelo.
    let cube_position = Vector3::new(0.0, 1.0, 0.0);

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();

        // ESPACIO permite congelar la luz para estudiar el resultado.
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            animate_light = !animate_light;
        }

        if animate_light {
            light_angle += delta_time * 0.8;
            light_position.x = light_angle.cos() * 3.5;
            light_position.z = light_angle.sin() * 3.5;
        }

        // La rotacion lenta hace que el reflejo recorra las distintas caras.
        rotation_angle += delta_time * 18.0;

        // Un brillo amplio y debil simula una superficie mas rugosa.
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            shininess = 4.0;
            specular_strength = 0.18;
            material_name = "RUGOSO / MATE";
        }

        // Un brillo concentrado e intenso simula una superficie lisa.
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            shininess = 24.0;
            specular_strength = 2.2;
            material_name = "LISO / PULIDO";
        }

        lighting_shader.set_shader_value(light_position_location, light_position);
        lighting_shader.set_shader_value(shininess_location, shininess);
        lighting_shader.set_shader_value(specular_strength_location, specular_strength);

        let mut drawing = rl.begin_drawing(&thread);
        drawing.clear_background(Color::new(8, 12, 22, 255));

        {
            // Las figuras de este bloque se proyectan usando la camara 3D.
            let mut mode_3d = drawing.begin_mode3D(camera);

            mode_3d.draw_model_ex(
                &mut cube_model,
                cube_position,
                Vector3::new(0.0, 1.0, 0.0),
                rotation_angle,
                Vector3::new(1.0, 1.0, 1.0),
                Color::WHITE,
            );

            // Este cubo pequeno muestra la posicion de la luz.
            mode_3d.draw_cube(light_position, 0.25, 0.25, 0.25, Color::YELLOW);

            // El suelo y la cuadricula dan una referencia clara de profundidad.
            mode_3d.draw_plane(
                Vector3::new(0.0, -0.02, 0.0),
                Vector2::new(12.0, 12.0),
                Color::new(28, 34, 48, 255),
            );
            mode_3d.draw_grid(12, 1.0);
        }

        drawing.draw_text(
            "CUBO CON LUZ DIFUSA Y ESPECULAR",
            20,
            20,
            22,
            Color::RAYWHITE,
        );
        drawing.draw_text(
            "1: rugoso/mate  |  2: liso/pulido  |  ESPACIO: pausar luz",
            20,
            52,
            18,
            Color::LIGHTGRAY,
        );
        drawing.draw_text(
            &format!("Material actual: {material_name}"),
            20,
            78,
            18,
            Color::ORANGE,
        );
        drawing.draw_text("ESC: salir", 20, HEIGHT - 35, 18, Color::LIGHTGRAY);
        drawing.draw_fps(WIDTH - 95, 10);
    }
}
