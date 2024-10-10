use nalgebra_glm::Vec3;
use std::f32::consts::PI;
pub struct Camera {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
    has_changed: bool,
}

const PITCH_LIMIT: f32 = PI / 2.0 - 0.1;

impl Camera {
    pub fn new(eye:Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed: true,
        }
    }

    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward); // Ya está normalizado
    
        let rotated =
            vector.x * right +
            vector.y * up -
            vector.z * forward;
    
        rotated
    }    

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();
        let radius_xz = radius_vector.xy().norm();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        let cos_pitch = new_pitch.cos();
        let new_eye = self.center + Vec3::new(
            radius * new_yaw.cos() * cos_pitch,
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * cos_pitch
        );

        self.eye = new_eye;
        self.has_changed = true;
    }

    pub fn zoom(&mut self, delta: f32) {
        let direction = (self.center - self.eye).normalize();
        self.eye += direction * delta;
        self.has_changed = true;
    }

    pub fn is_changed(&mut self) -> bool {
        if self.has_changed {
            self.has_changed = false;
            return true;
        }
        false
    }
}