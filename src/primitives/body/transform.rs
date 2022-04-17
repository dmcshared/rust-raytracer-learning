use crate::primitives::{intersection::Intersection, matrix::Matrix4f, ray::Ray};

use super::Body;

pub struct TransformedBody<T>
where
    T: Body,
{
    pub transformation: Matrix4f,
    pub raw_body: T,
}

impl<T> TransformedBody<T>
where
    T: Body,
{
    pub fn new_with_body(transformation: Matrix4f, raw_body: T) -> Self {
        Self {
            transformation,
            raw_body,
        }
    }

    pub fn new(transformation: Matrix4f) -> Self {
        Self {
            transformation,
            raw_body: T::default(),
        }
    }
}

impl<T> Body for TransformedBody<T>
where
    T: Body,
{
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = (self
            .transformation
            .inverse()
            .expect("Transform Matrix in TransformedBody must be inversible"))
            * ray;
        self.raw_body.intersect(&local_ray)
    }

    fn default() -> Self {
        Self::new(Matrix4f::identity())
    }
}
