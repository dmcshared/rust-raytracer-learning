use super::{
    intersection::Intersection,
    material::Material,
    ray::Ray,
    three_part::{point::Point, vector::Vector},
};

pub mod scene;
pub mod sphere;
pub mod transform;
use std::{fmt::Debug, sync::Arc};

pub trait Body: Debug + Sync {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn normal_raw(&self, x: f64, y: f64, z: f64) -> Vector;
    fn normal(&self, p: Point) -> Vector {
        self.normal_raw(p.0 .0, p.0 .1, p.0 .2)
    }
    fn get_material(&self) -> Arc<dyn Material>;
}

pub trait BodyBuilder {
    fn with_material(&self, material: Arc<dyn Material>) -> Self;
}
