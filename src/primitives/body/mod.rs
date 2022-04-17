use super::{
    intersection::Intersection,
    ray::Ray,
    three_part::{point::Point, vector::Vector},
};

pub mod sphere;
pub mod transform;

pub trait Body {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn normal_raw(&self, x: f64, y: f64, z: f64) -> Vector;
    fn normal(&self, p: Point) -> Vector {
        self.normal_raw(p.0 .0, p.0 .1, p.0 .2)
    }
}
