use core::f32;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::time::{Duration, Instant};
use std::f32::consts::PI;
use std::f32::INFINITY;
use rand::Rng;

mod framebuffer;
use framebuffer::Framebuffer;

mod cube;
use cube::Cube;

mod ray_intersect;
use ray_intersect::{Intersect, RayIntersect};

mod color;
use color::Color;

mod camera;
use camera::Camera;

mod material;
use material::Material;

mod light;
use light::Light;

mod texture;
use std::sync::Arc;
use texture::Texture;

const BIAS: f32 = 0.001;
const SKYBOX_COLOR: Color = Color::new(135, 206, 235); // Light sky blue

const AMBIENT_LIGHT_COLOR: Color = Color::new(50, 50, 50);
const AMBIENT_INTENSITY: f32 = 0.3;

fn offset_point(intersect: &Intersect, direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * BIAS;
    intersect.point + offset
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_shadow(
    intersect: &Intersect,
    objects: &[Cube],
    light_dir: &Vec3,
    light_distance: f32
) -> f32 {
    let shadow_ray_origin = offset_origin(intersect, light_dir);
    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            // Si el objeto intersectado emite luz, reduce la sombra, pero no la elimina completamente
            if let Some(_emission) = object.material.emission_color {
                let distance_ratio = shadow_intersect.distance / light_distance;
                let emission_intensity = 1.0 / (distance_ratio * distance_ratio);
                shadow_intensity = emission_intensity; // Ajustar la sombra según la intensidad de la emisión
                break; // Asumimos que el bloque emisor de luz bloquea cualquier otra sombra
            } else {
                // Si no es un emisor de luz, aplica la sombra normalmente
                shadow_intensity = 1.0;
                break;
            }
        }
    }

    shadow_intensity
}

fn get_skybox_color(ray_direction: &Vec3, skybox: &Texture) -> Color {
    let dir = ray_direction.normalize();
    let u = 0.5 + (dir.x.atan2(dir.z) / (2.0 * PI));
    let v = 0.5 - (dir.y.asin() / PI);
    skybox.get_color_at_uv(u, v)
}

fn clamp_color(color: Color) -> Color {
    Color::new(
        color.r().min(255).max(0),
        color.g().min(255).max(0),
        color.b().min(255).max(0),
    )
}

fn generate_random_direction() -> Vec3 {
    let theta = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
    let z: f32 = rand::random::<f32>() * 2.0 - 1.0;  // Random valor entre -1 y 1
    let r = (1.0 - z * z).sqrt();
    let x = r * theta.cos();
    let y = r * theta.sin();
    Vec3::new(x, y, z).normalize()
}

fn offset_origin(intersect: &Intersect, direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * BIAS;
    if direction.dot(&intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}


pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Cube],
    lights: &[Light],
    skybox: &Texture,
    depth: u32,
) -> Color {
    if depth >= 3 {
        return SKYBOX_COLOR;
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = INFINITY;

    // Comprobación de intersección con los objetos
    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

    if !intersect.is_intersecting {
        return get_skybox_color(ray_direction, skybox);
    }

    let ambient_light = AMBIENT_LIGHT_COLOR * AMBIENT_INTENSITY;
    let mut total_light = ambient_light;

    // Calcular la luz total desde las luces
    for light in lights {
        let light_dir = (light.position - intersect.point).normalize();
        let light_distance = (light.position - intersect.point).magnitude();
        let view_dir = (ray_origin - intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();

        // Calcular la intensidad de sombra para esta luz usando cast_shadow
        let shadow_intensity = cast_shadow(&intersect, objects, &light_dir, light_distance);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

        // Cálculo de la luz difusa
        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
        let diffuse_color = intersect.material.get_diffuse_color(intersect.u, intersect.v);
        let diffuse = diffuse_color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

        // Cálculo de la luz especular
        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
        let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

        total_light = total_light + diffuse + specular;
    }

    // Añadir la luz de emisión
    let mut emission_contribution = Color::black();
    for object in objects {
        if let Some(emission) = object.material.emission_color {
            let num_rays = 16;  // Número de direcciones para emitir luz
            let emission_strength = 1.0 / (num_rays as f32);  // Reducir la intensidad de emisión

            for _ in 0..num_rays {
                let emission_dir = generate_random_direction();
                let emission_origin = object.position();
                let emission_distance = (emission_origin - intersect.point).magnitude();
                let emission_intensity = emission_strength / (1.0 + emission_distance * emission_distance);

                let emission_diffuse_intensity = intersect.normal.dot(&emission_dir).max(0.0);
                let emission_diffuse = emission * emission_diffuse_intensity * emission_intensity;

                emission_contribution = emission_contribution + emission_diffuse;
            }
        }
    }

    // Sumar la contribución de emisión a la luz total
    total_light = total_light + emission_contribution;

    // Clampeo del color final
    total_light = clamp_color(total_light);
    total_light
}



pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, lights: &[Light], current_skybox: &Arc<Texture>) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov / 2.0).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;
            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let rotated_direction = camera.basis_change(&ray_direction);
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, lights, &current_skybox, 0);
            framebuffer.set_current_color(pixel_color.to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Diorama Casa con Césped",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    // Definiendo texturas y materiales
    let grass_texture: Arc<Texture> = Arc::new(Texture::new("assets/grass_texture.png"));
    let wood_texture: Arc<Texture> = Arc::new(Texture::new("assets/wood_texture.png"));
    let door_texture: Arc<Texture> = Arc::new(Texture::new("assets/dark_door_texture.png"));
    let glass_texture: Arc<Texture> = Arc::new(Texture::new("assets/glass_texture.png"));
    let plank_texture: Arc<Texture> = Arc::new(Texture::new("assets/plank.png"));
    let stone_texture: Arc<Texture> = Arc::new(Texture::new("assets/stone_texture.jpg"));
    let glowstone_texture: Arc<Texture> = Arc::new(Texture::new("assets/glowstone_texture.jpeg"));

    let grass_material = Material::new_with_texture(
        0.1,
        [0.8, 0.1, 0.0, 0.0],
        1.0,
        grass_texture.clone(),
        None,
        0.0,
    );
    let wood_material = Material::new_with_texture(
        0.2,
        [0.9, 0.05, 0.0, 0.0],
        1.0,
        wood_texture.clone(),
        None,
        0.0,
    );
    let plank_material = Material::new_with_texture(
        0.2,
        [0.9, 0.05, 0.0, 0.0],
        1.0,
        plank_texture.clone(),
        None,
        0.0,
    );
    let stone_material = Material::new_with_texture(
        0.2,
        [0.9, 0.05, 0.0, 0.0],
        1.0,
        stone_texture.clone(),
        None,
        0.0,
    );
    let door_material = Material::new_with_texture(
        0.3,
        [0.7, 0.1, 0.0, 0.0],
        1.0,
        door_texture.clone(),
        None,
        0.0,
    );
    let glass_material = Material::new_with_texture(
        0.3,
        [0.7, 0.1, 0.0, 0.5],  // Puedes ajustar los valores de albedo si es necesario
        1.5,  // Ajusta el índice de refracción a 1.5 para el vidrio
        glass_texture.clone(),
        None,
        0.0,
    );
    let glowstone_texture = Material::new_with_texture(
        50.0,                        // Specular
        [0.9, 0.1, 0.0, 0.0],        // Albedo
        0.0,                   // Refractive index
        glowstone_texture.clone(),        // Textura para el material
        Some(Color::new(255, 255, 0)),  // Color de emisión
        1.0
    );

    // Base de césped 9x8
    let mut objects: Vec<Cube> = Vec::new();
    for i in 0..9 {
        for j in 0..8 {
            objects.push(Cube {
                min: Vec3::new(i as f32, -1.0, j as f32),
                max: Vec3::new(i as f32 + 1.0, 0.0, j as f32 + 1.0),
                material: grass_material.clone(),
            });
        }
    }

    // Base y paredes de la casa con columnas de wood_material, paredes de plank_material y capa superior de stone_material
    for i in 1..8 {  // Base de 7 bloques de ancho
        for j in 2..6 {  // Base de 4 bloques de profundidad
            // Determinar si es una columna (esquinas de la casa)
            let is_column = (i == 1 || i == 7) && (j == 2 || j == 5);

            // Evitar la creación de bloques donde va la puerta (posición [4, 3])
            let is_door_position = (i == 4 && j == 5); // Ajustar la posición a la nueva altura de la puerta

            // Primer bloque de altura (base)
            if !is_door_position {
                objects.push(Cube {
                    min: Vec3::new(i as f32, 0.0, j as f32),
                    max: Vec3::new(i as f32 + 1.0, 1.0, j as f32 + 1.0),
                    material: if is_column {
                        wood_material.clone()  // Usar wood_material para las columnas
                    } else {
                        plank_material.clone() // Usar plank_material para las paredes
                    },
                });
            }

            // Bloques de altura adicionales (paredes y columnas) hasta una altura de 5
            for k in 1..4 {
                // Verificar si es la última capa (k == 3)
                let material = if k == 3 {
                    stone_material.clone()  // Usar stone_material para la última capa
                } else if is_column {
                    wood_material.clone()  // Usar wood_material para las columnas
                } else {
                    plank_material.clone() // Usar plank_material para las paredes
                };

                // Evitar poner bloques donde van las ventanas y la puerta
                if !(i == 3 && j == 5 && k == 1) &&  // Ventana 1
                !(i == 5 && j == 5 && k == 1) &&  // Ventana 2
                !(i == 4 && j == 5 && k < 2) {    // Evitar bloques en la puerta (altura hasta 2)
                    
                    objects.push(Cube {
                        min: Vec3::new(i as f32, k as f32, j as f32),
                        max: Vec3::new(i as f32 + 1.0, k as f32 + 1.0, j as f32 + 1.0),
                        material: material,  // Asignar el material dependiendo de la capa
                    });
                }
            }
        }
    }

    // Ventanas en el segundo bloque de altura (k = 1)
    objects.push(Cube {
        min: Vec3::new(3.0, 1.0, 5.0),
        max: Vec3::new(4.0, 2.0, 6.0),
        material: glass_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(5.0, 1.0, 5.0),
        max: Vec3::new(6.0, 2.0, 6.0),
        material: glass_material.clone(),
    });
    objects.push(Cube {
        min: Vec3::new(7.0, 0.0, 6.0),
        max: Vec3::new(8.0, 1.0, 7.0),
        material: glowstone_texture.clone(),
    });

    // Puerta en el centro con altura de 3 bloques
    objects.push(Cube {
        min: Vec3::new(4.0, 0.0, 5.0),
        max: Vec3::new(5.0, 2.0, 6.0), 
        material: door_material.clone(),
    });
    
    // Inicializando la cámara
    let mut camera = Camera::new(
        Vec3::new(10.0, 10.0, 20.0),
        Vec3::new(4.0, 0.0, 4.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let lights = vec![Light::new(Vec3::new(-10.0, 10.0, 10.0), Color::new(255, 255, 255), 1.0)];

    let skybox_texture = Arc::new(Texture::new("assets/sky.jpeg"));
    let skybox_night_texture = Arc::new(Texture::new("assets/night_texture.jpg"));
    let mut current_skybox_texture = skybox_texture.clone();

    let daytime_light = Light::new(Vec3::new(-10.0, 10.0, 10.0), Color::new(255, 255, 255), 1.0); // Luz brillante
    let nighttime_light = Light::new(Vec3::new(-10.0, 10.0, 10.0), Color::new(10, 10, 10), 0.5); // Luz más tenue y azulada

    let mut current_light = daytime_light.clone(); // Inicialmente la luz diurna

    // Ciclo principal
    let mut previous_time = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(previous_time).as_secs_f32();
        previous_time = current_time;

        framebuffer.clear();
        if window.is_key_down(Key::D) {
            current_skybox_texture = skybox_texture.clone(); // Cambiar a cielo diurno
            current_light = daytime_light.clone(); // Cambiar a luz diurna
        } else if window.is_key_down(Key::N) {
            current_skybox_texture = skybox_night_texture.clone(); // Cambiar a cielo nocturno
            current_light = nighttime_light.clone(); // Usar luz nocturna
        }

        render(&mut framebuffer, &objects, &camera, &[current_light.clone()], &current_skybox_texture);
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        // Control de la cámara y movimiento
        if window.is_key_down(Key::Left) {
            camera.orbit(PI / 10.0 * delta_time, 0.0);
        }
        if window.is_key_down(Key::Right) {
            camera.orbit(-PI / 10.0 * delta_time, 0.0);
        }
        if window.is_key_down(Key::Up) {
            camera.zoom(0.5 * delta_time);
        }
        if window.is_key_down(Key::Down) {
            camera.zoom(-0.5 * delta_time);
        }

        std::thread::sleep(frame_delay);
    }
}
