use std::sync::Arc;

use crate::primitives::{intersection::Intersection, ray::Ray};

use super::Body;

#[derive(Debug, Clone)]
pub struct Scene {
    pub bodies: Vec<Arc<dyn Body>>,
}

impl Scene {
    pub fn new(bodies: Vec<Arc<dyn Body>>) -> Arc<Self> {
        // Arc::new_cyclic(|weak| Self { bodies })
        Arc::new(Self { bodies })
    }
}

impl Body for Scene {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        self.bodies.iter().flat_map(|b| b.intersect(ray)).collect()
    }

    fn normal_raw(
        &self,
        _x: f64,
        _y: f64,
        _z: f64,
    ) -> crate::primitives::three_part::vector::Vector {
        panic!(
            "Something is calling normal_raw on a Scene. Intersection.object should not be Scene"
        )
    }

    fn get_material(&self) -> Arc<dyn crate::primitives::material::Material> {
        panic!(
            "Something called get_material on Scene. Only call get_material on Intersection.object"
        )
    }
}
