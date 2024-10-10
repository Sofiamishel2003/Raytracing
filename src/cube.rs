use crate::ray_intersect::RayIntersect;
use nalgebra_glm::Vec3;
use crate::material::Material;

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Self {
        Cube { min, max, material }
    }

    pub fn intersect(&self, origin: &Vec3, direction: &Vec3) -> Option<Intersect> {
        let inv_dir = 1.0 / direction;
        let t0 = (self.min - origin).component_mul(&inv_dir);
        let t1 = (self.max - origin).component_mul(&inv_dir);
        
        let tmin = t0.min(&t1).max();
        let tmax = t0.max(&t1).min();
        
        if tmax >= tmin && tmax >= 0.0 {
            Some(Intersect {
                point: origin + direction * tmin,
                normal: self.get_normal(tmin, tmax),
                distance: tmin,
                material: self.material.clone(),
                is_intersecting: true,
                ..Default::default() // Asegúrate de tener un `default` para campos faltantes si es necesario.
            })
        } else {
            None
        }
    }

    fn get_normal(&self, tmin: f32, tmax: f32) -> Vec3 {
        // Lógica para obtener la normal adecuada dependiendo del tmin/tmax y la cara del cubo que se intersecta
        Vec3::new(0.0, 1.0, 0.0) // Aquí podrías calcular la normal real del cubo.
    }
}

impl RayIntersect for Cube {
    fn intersect(&self, origin: &Vec3, direction: &Vec3) -> Option<Intersect> {
        // Implementación de la lógica de intersección de rayos
        let inv_dir = 1.0 / direction;
        let t0 = (self.min - origin).component_mul(&inv_dir);
        let t1 = (self.max - origin).component_mul(&inv_dir);

        let tmin = t0.min(&t1).max();
        let tmax = t0.max(&t1).min();

        if tmax >= tmin && tmax >= 0.0 {
            Some(Intersect {
                point: origin + direction * tmin,
                normal: self.get_normal(tmin, tmax),
                distance: tmin,
                material: self.material.clone(),
                is_intersecting: true,
                ..Default::default()
            })
        } else {
            None
        }
    }
}
