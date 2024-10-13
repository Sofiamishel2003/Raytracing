use core::f32;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::time::Duration;
use std::f32::consts::PI;
use std::f32::INFINITY;

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

fn cast_shadow(intersect: &Intersect, light: &Light, objects: &[Cube]) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let light_distance = (light.position - intersect.point).magnitude();

    let shadow_ray_origin = offset_point(intersect, &light_dir);
    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            shadow_intensity = 1.0;
            break;
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

    for light in lights {
        let light_dir = (light.position - intersect.point).normalize();
        let view_dir = (ray_origin - intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();

        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
        let diffuse_color = intersect.material.get_diffuse_color(intersect.u, intersect.v);
        let diffuse = diffuse_color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
        let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

        total_light = total_light + diffuse + specular;
    }

    total_light = clamp_color(total_light);
    total_light
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, lights: &[Light]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov / 2.0).tan();
    let skybox_texture = Arc::new(Texture::new("assets/sky.jpeg"));

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;
            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let rotated_direction = camera.basis_change(&ray_direction);
            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, lights, &skybox_texture, 0);
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
        "Diorama Minecraft",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    let skybox_texture = Arc::new(Texture::new("assets/sky.jpeg"));

    let mut objects: Vec<Cube> = Vec::new();
    
    let stone_texture: Arc<Texture> = Arc::new(Texture::new("assets/stone_texture.png"));
    let dirt_texture: Arc<Texture> = Arc::new(Texture::new("assets/wood_texture.png"));

    let stone_material = Material::new_with_texture(
        0.2,                      // specular
        [0.8, 0.1, 0.0, 0.0],     // albedo
        1.3,                      // refractive_index
        stone_texture.clone(),     // texture
        None,                      // emission_color (None en este caso)
        0.0                        // emission_intensity
    );
    let dirt_material = Material::new_with_texture(
        0.1,                      // specular
        [0.9, 0.05, 0.0, 0.0],    // albedo
        1.0,                      // refractive_index
        dirt_texture.clone(),      // texture
        None,                      // emission_color (None en este caso)
        0.0                        // emission_intensity
    );
    // Construyendo una pequeña casa en el centro del escenario
    for i in 0..2 {
        for j in 0..2 {
            objects.push(Cube {
                min: Vec3::new(i as f32, 0.0, j as f32),
                max: Vec3::new(i as f32 + 1.0, 1.0, j as f32 + 1.0),
                material: stone_material.clone(),
            });

            objects.push(Cube {
                min: Vec3::new(i as f32, -1.0, j as f32),
                max: Vec3::new(i as f32 + 1.0, 0.0, j as f32 + 1.0),
                material: dirt_material.clone(),
            });
        }
    }

    let mut camera = Camera::new(
        Vec3::new(5.0, 5.0, 10.0), 
        Vec3::new(0.0, 0.0, 0.0),   
        Vec3::new(0.0, 1.0, 0.0),  
    );

    let lights = vec![Light::new(Vec3::new(-10.0, 10.0, 10.0), Color::new(255, 255, 255), 1.0)];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear();
        render(&mut framebuffer, &objects, &camera, &lights);
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}
