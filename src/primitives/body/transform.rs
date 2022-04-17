use crate::primitives::{
    intersection::Intersection, matrix::Matrix4f, ray::Ray, three_part::point::Point,
};

use super::{Body, BodyBuilder};

pub struct TransformedBody<T>
where
    T: Body,
{
    transformation: Matrix4f,
    inverse_transformation: Matrix4f,
    transpose_inverse_transformation: Matrix4f,
    pub raw_body: T,
}

impl<T> TransformedBody<T>
where
    T: Body,
    T: Default,
{
    pub fn new(transformation: Matrix4f) -> Self {
        Self::new_with_body(transformation, T::default())
    }
}

impl<T> TransformedBody<T>
where
    T: Body,
{
    pub fn new_with_body(transformation: Matrix4f, raw_body: T) -> Self {
        let inverse_transformation = transformation
            .inverse()
            .expect("Transform Matrix in TransformedBody must be inversible");
        Self {
            transformation,
            raw_body,
            inverse_transformation,
            transpose_inverse_transformation: inverse_transformation.transpose().fix_transform(),
        }
    }

    pub fn set_transformation(&mut self, transformation: Matrix4f) {
        self.transformation = transformation;
        self.inverse_transformation = transformation
            .inverse()
            .expect("Transform Matrix in TransformedBody must be inversible");
        self.transpose_inverse_transformation =
            self.inverse_transformation.transpose().fix_transform();
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

    fn normal_raw(&self, x: f64, y: f64, z: f64) -> crate::primitives::three_part::vector::Vector {
        let local_point = self.inverse_transformation * Point::new(x, y, z);
        let local_normal = self.raw_body.normal(local_point);
        (self.transpose_inverse_transformation * local_normal).normalize()
    }

    fn get_material(&self) -> Box<dyn crate::primitives::material::Material> {
        self.raw_body.get_material()
    }
}

impl<T> BodyBuilder for TransformedBody<T>
where
    T: Body,
    T: BodyBuilder,
{
    fn with_material(
        &self,
        material: Box<dyn crate::primitives::material::Material>,
    ) -> TransformedBody<T> {
        TransformedBody::<T>::new_with_body(
            self.transformation,
            self.raw_body.with_material(material),
        )
    }
}

impl<T> Default for TransformedBody<T>
where
    T: Body,
    T: Default,
{
    fn default() -> Self {
        Self::new(Matrix4f::identity())
    }
}
