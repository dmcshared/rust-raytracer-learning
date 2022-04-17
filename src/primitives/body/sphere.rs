use crate::primitives::{intersection::Intersection, ray::Ray, three_part::point::Point};

use super::{transform::TransformedBody, Body};

#[derive(Debug, Copy, Clone)]
pub struct RawSphere {}

impl Body for RawSphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Point::origin(); // (0, 0, 5)
        let a = ray.direction.sqr_magnitude(); // 1
        let b = 2.0 * (ray.direction * sphere_to_ray); // 10
        let c = sphere_to_ray.sqr_magnitude() - 1.0; // 24

        // let discriminant = (ray.direction * sphere_to_ray).powi(2) - c; // 5 - 24 = -19
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else if discriminant == 0.0 {
            let t = -b / (2.0 * a);
            vec![Intersection::new(t, Box::new(*self))]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![
                Intersection {
                    t: t1,
                    object: Box::new(*self),
                },
                Intersection {
                    t: t2,
                    object: Box::new(*self),
                },
            ]
        }
    }

    fn default() -> Self {
        RawSphere {}
    }
}

pub type Sphere = TransformedBody<RawSphere>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{
        intersection::IntersectionList, matrix::Matrix4f, three_part::vector::Vector,
    };
    use crate::primitives::{ray::Ray, three_part::point::Point};

    #[test]
    fn test_sphere_intersect() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn test_sphere_hits_negative() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn test_sphere_misses() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_sphere_behind() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, -1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn test_sphere_inside() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn test_sphere_tangent() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 1.0, -1.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
    }

    #[test]
    fn test_translated_sphere() {
        let s = Sphere::new(Matrix4f::translate_raw(2.0, 0.0, 0.0));
        let r = Ray::new(Point::new(2.0, 0.0, -3.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 2.0);
        assert_eq!(xs[1].t, 4.0);
    }

    #[test]
    fn test_hit_method() {
        let s = Sphere::new(Matrix4f::identity());
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = s.intersect(&r);

        let x = xs
            .hit()
            .expect("This should contain 2 intersections, so a hit should always exist.");

        assert_eq!(x.t, 4.0);
    }
}
