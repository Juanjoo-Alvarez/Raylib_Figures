# Esfera 3D con Raylib

**Estudiante:** Juan Jose Rivas Alvarez  
**Carnet universitario:** 24856

Esta version dibuja una esfera dentro de un mundo 3D de Raylib. Utiliza una
camara en perspectiva, una posicion tridimensional para la esfera y una
cuadricula que permite apreciar la profundidad.

## Ejecucion

Se necesita Rust instalado. Desde esta carpeta, ejecutar:

```powershell
cargo run
```

La primera compilacion puede tardar porque Cargo debe descargar y compilar
Raylib. En Windows tambien puede ser necesario tener instaladas las herramientas
de compilacion de C/C++ de Visual Studio.

## Control

- `ESC`: cierra la ventana.

## Elementos 3D

- `Vector3`: representa posiciones y direcciones mediante `(x, y, z)`.
- `Camera3D`: determina desde donde se observa el mundo.
- `draw_sphere`: dibuja la esfera en su posicion 3D.
- `draw_grid`: dibuja una referencia visual sobre el suelo.

La posicion de la esfera se define de esta manera:

```rust
let sphere_position = Vector3::new(0.0, 1.0, 0.0);
```

Raylib proyecta la geometria 3D sobre la ventana 2D.
