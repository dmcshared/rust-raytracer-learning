use crate::primitives::{intersection::Intersection, matrix::Matrix4f, ray::Ray};

use super::Body;

pub struct TransformedBody<T>
where
    T: Body,
{
    transformation: Matrix4f,
    inverse_transformation: Matrix4f,
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
            inverse_transformation: transformation
                .inverse()
                .expect("Transform Matrix in TransformedBody must be inversible"),
        }
    }

    pub fn new(transformation: Matrix4f) -> Self {
        Self {
            transformation,
            raw_body: T::default(),
            inverse_transformation: transformation
                .inverse()
                .expect("Transform Matrix in TransformedBody must be inversible"),
        }
    }

    pub fn set_transformation(&mut self, transformation: Matrix4f) {
        self.transformation = transformation;
        self.inverse_transformation = transformation
            .inverse()
            .expect("Transform Matrix in TransformedBody must be inversible");
    }
}

impl<T> Body for TransformedBody<T>
where
    T: Body,
{
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = self.inverse_transformation * ray;
        self.raw_body.intersect(&local_ray)
    }

    fn default() -> Self {
        Self::new(Matrix4f::identity())
    }
}
