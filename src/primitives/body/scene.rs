use std::sync::{Arc, RwLock, Weak};

use crate::{
    primitives::{intersection::Intersection, ray::Ray},
    util::weak_cell::WeakCell,
};

use super::Body;

#[derive(Debug, Clone)]
pub struct Scene {
    pub bodies: Vec<Arc<dyn Body>>,
    arc_scene: Weak<Scene>,
}

impl Scene {
    pub fn new(bodies: Vec<Arc<dyn Body>>) -> Arc<Self> {
        Arc::new_cyclic(|weak| Self {
            bodies,
            arc_scene: *weak,
        })
    }
}

impl Body for Scene {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        self.bodies
            .iter()
            .flat_map(|b| b.intersect(ray))
            .map(|b| {
                b.with_top_level_object(
                    self.arc_scene
                        .as_ref()
                        .expect("arc_scene should be defined when using new")
                        .upgrade()
                        .expect("arc_scene is self, so it should always upgrade in self's methods"),
                )
            })
            .collect()
    }

    fn normal_raw(&self, x: f64, y: f64, z: f64) -> crate::primitives::three_part::vector::Vector {
        todo!()
    }

    fn get_material(&self) -> Arc<dyn crate::primitives::material::Material> {
        todo!()
    }
}
