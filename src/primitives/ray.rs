use std::ops;

use crate::prelude::body::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

impl ops::Mul<Ray> for Matrix4f {
    type Output = Ray;

    fn mul(self, ray: Ray) -> Self::Output {
        let origin = self * ray.origin;
        let direction = self * ray.direction;
        Ray::new(origin, direction)
    }
}

impl ops::Mul<&Ray> for Matrix4f {
    type Output = Ray;

    fn mul(self, ray: &Ray) -> Self::Output {
        let origin = self * ray.origin;
        let direction = self * ray.direction;
        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ray_new() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction.normalize());
    }

    #[test]
    fn ray_position() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(r.at(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(r.at(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(r.at(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(r.at(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix4f::translate_raw(3.0, 4.0, 5.0);

        let r2 = m * r;
        assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix4f::scale_raw(2.0, 3.0, 4.0);

        let r2 = m * r;
        assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
    }
}
