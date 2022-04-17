use super::{intersection::Intersection, ray::Ray};

pub mod sphere;
pub mod transform;

pub trait Body {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;

    fn default() -> Self
    where
        Self: Sized;
}
