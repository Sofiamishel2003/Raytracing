use nalgebra_glm::{Vec3};
use std::sync::Arc;
use minifb::{Key, Window, WindowOptions};
use crate::camera::Camera;
use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::cube::Cube;
use crate::material::Material;
use crate::texture::Texture;
use crate::ray_intersect::Intersect;

mod camera;
mod color;
mod framebuffer;
mod light;
mod cube;
mod material;
mod texture;
mod ray_intersect;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut window = Window::new("Diorama", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let sky_color = Color::new(68, 142, 228);

    // Create textures for cubes
    let wood_texture = Arc::new(Texture::new("wood.png"));
    let stone_texture = Arc::new(Texture::new("stone.png"));
    let glass_texture = Arc::new(Texture::new("glass.png"));
    let emissive_texture = Arc::new(Texture::new("emissive.png"));

    // Materials
    let wood_material = Material::new_with_texture(
        50.0,
        [0.9, 0.1, 0.0, 0.0],
        0.0,
        Some(wood_texture.clone()),
    );
    let stone_material = Material::new_with_texture(
        50.0,
        [0.9, 0.1, 0.0, 0.0],
        0.0,
        Some(stone_texture.clone()),
    );
    let glass_material = Material::new_with_texture(
        50.0,
        [0.9, 0.1, 0.0, 0.0],
        0.0,
        Some(glass_texture.clone()),
    );
    let emissive_material = Material::new_with_texture(
        50.0,
        [0.9, 0.1, 0.0, 0.0],
        0.0,
        Some(emissive_texture.clone()),
    );

    // Create the cubes for the house structure
    let house_blocks = vec![
        Cube::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0), wood_material.clone()),
        Cube::new(Vec3::new(1.0, -1.0, -1.0), Vec3::new(3.0, 1.0, 1.0), stone_material.clone()),
        Cube::new(Vec3::new(-1.0, -1.0, 1.0), Vec3::new(1.0, 1.0, 3.0), glass_material.clone()),
        Cube::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(3.0, 1.0, 3.0), emissive_material.clone()),
    ];

    let mut objects = Vec::new();
    objects.extend(house_blocks);

    // Add a light source
    let light = Light::new(
        Vec3::new(5.0, 5.0, 5.0),
        Color::new(255, 255, 255),
        1.0,
    );

    // Create the camera
    let mut camera = Camera::new(
        Vec3::new(0.0, 5.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
    );

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the framebuffer
        framebuffer.clear(sky_color);

        // Render the scene
        render(&mut framebuffer, &objects, &camera, &light, &sky_color);

        // Update camera controls
        if window.is_key_down(Key::W) {
            camera.zoom(0.1);
        }
        if window.is_key_down(Key::S) {
            camera.zoom(-0.1);
        }

        // Display the framebuffer
        window.update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Cube],
    camera: &Camera,
    light: &Light,
    sky_color: &Color,
) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let direction = camera.pixel_direction(x, y, WIDTH, HEIGHT);
            let rotated_direction = camera.rotate_direction(&direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light, 0, sky_color);
            framebuffer.put_pixel(x, y, pixel_color);
        }
    }
}

fn cast_ray(
    origin: &Vec3,
    direction: &Vec3,
    objects: &[Cube],
    light: &Light,
    depth: u32,
    sky_color: &Color,
) -> Color {
    // Implement ray casting to intersect with objects, compute lighting
    // and return the computed color for the pixel
    let mut closest_intersect = Intersect::empty();
    let mut color = *sky_color;

    for object in objects {
        if let Some(intersect) = object.intersect(origin, direction) {
            if intersect.distance < closest_intersect.distance {
                closest_intersect = intersect;
                let diffuse_color = closest_intersect.material.get_diffuse_color(closest_intersect.u, closest_intersect.v);
                color = diffuse_color;
            }
        }
    }

    color
}
